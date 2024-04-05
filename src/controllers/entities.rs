use crate::{ApiClient, BatchObject};

use super::Entity;
use anyhow::{anyhow, Result};
use serde::Serialize;
use tokio::task::JoinSet;
const BATCH: &str = "batch";

impl ApiClient {
    pub async fn retrieve<T: Entity>(&self, entity_id: i32) -> Result<T> {
        let uri = self
            .base_url
            .join(&T::endpoint())?
            .join(&entity_id.to_string())?;
        for i in 1..3 {
            tracing::debug!("Connecting {uri}, try {i}");
            match self
                .client
                .get(uri.clone())
                .basic_auth(self.ck(), Some(self.cs()))
                .send()
                .await?
                .json::<T>()
                .await
            {
                Ok(r) => {
                    return Ok(r);
                }
                Err(e) => {
                    tracing::error!(
                        "Failed to connect to {uri} with error: {e}\n{} tries left",
                        3 - i
                    );
                    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
                    continue;
                }
            }
        }
        Err(anyhow!("Error retrieving entitity with id: {entity_id}"))
    }
    pub async fn list_all<T: Entity>(&self) -> Result<Vec<T>> {
        let uri = self.base_url.join(&T::endpoint())?;
        let mut result = Vec::new();
        let mut set = JoinSet::new();
        let total_response = self
            .client
            .get(uri.clone())
            .basic_auth(&self.ck(), Some(self.cs()))
            .send()
            .await?;
        let total = total_response
            .headers()
            .get("X-WP-Total")
            .and_then(|h| h.to_str().ok().map(|p| p.parse::<i32>().ok()))
            .flatten()
            .unwrap_or_default();
        let per_page = 50;
        let total_pages = total / per_page + 1;
        for page in 1..=total_pages {
            let client = self.client();
            let ck = self.ck();
            let cs = Some(self.cs());
            let url = uri.clone();
            set.spawn(async move {
                client
                    .get(url)
                    .query(&[("page", page), ("per_page", per_page)])
                    .basic_auth(ck, cs)
                    .send()
                    .await?
                    .json::<Vec<T>>()
                    .await
            });
        }
        while let Some(Ok(Ok(v))) = set.join_next().await {
            result.extend(v)
        }
        Ok(result)
    }
    pub async fn create<T: Entity>(&self, object: impl Serialize) -> Result<T> {
        let uri = self.base_url.join(&T::endpoint())?;
        for i in 1..3 {
            tracing::debug!("Connecting {uri}, try {i}");
            match self
                .client
                .post(uri.clone())
                .basic_auth(self.ck(), Some(self.cs()))
                .json(&object)
                .send()
                .await?
                .json::<T>()
                .await
            {
                Ok(r) => {
                    return Ok(r);
                }
                Err(e) => {
                    tracing::error!(
                        "Failed to connect to {uri} with error: {e}\n{} tries left",
                        5 - i
                    );
                    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                    continue;
                }
            }
        }
        Err(anyhow!("Error retrieving entitities"))
    }
    pub async fn update<T: Entity>(&self, entity_id: i32, object: impl Serialize) -> Result<T> {
        let uri = self
            .base_url
            .join(&T::endpoint())?
            .join(&entity_id.to_string())?;
        for i in 1..3 {
            tracing::debug!("Connecting {uri}, try {i}");
            match self
                .client
                .put(uri.clone())
                .basic_auth(self.ck(), Some(self.cs()))
                .json(&object)
                .send()
                .await?
                .json::<T>()
                .await
            {
                Ok(r) => return Ok(r),
                Err(e) => {
                    tracing::error!(
                        "Failed to connect to {uri} with error: {e}\n{} tries left",
                        3 - i
                    );
                    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                    continue;
                }
            }
        }
        Err(anyhow!("Error updating entitity with id: {entity_id}"))
    }
    pub async fn delete<T: Entity>(&self, entity_id: i32) -> Result<T> {
        let uri = self
            .base_url
            .join(&T::endpoint())?
            .join(&entity_id.to_string())?;
        for i in 1..3 {
            tracing::debug!("Connecting {uri}, try {i}");
            match self
                .client
                .delete(uri.clone())
                .basic_auth(self.ck(), Some(self.cs()))
                .send()
                .await?
                .json::<T>()
                .await
            {
                Ok(r) => {
                    return Ok(r);
                }
                Err(e) => {
                    tracing::error!(
                        "Failed to connect to {uri} with error: {e}\n{} tries left",
                        3 - i
                    );
                    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                    continue;
                }
            }
        }
        Err(anyhow!("Error deleting entitity with id: {entity_id}"))
    }
    pub async fn search<T: Entity, S: ToString>(&self, _search_string: S) -> Result<Vec<T>> {
        let result = vec![];
        Ok(result)
    }
    pub async fn batch_create<T: Entity, O: Serialize + Clone + Send + 'static>(
        &self,
        create_objects: Vec<O>,
    ) -> Result<Vec<T>> {
        let mut result = Vec::new();
        let mut set = JoinSet::new();
        let uri = self.base_url.join(&T::endpoint())?.join(BATCH)?;
        let batched = create_objects
            .chunks(100)
            .map(|c| BatchObject::builder().extend_create(c.to_vec()).build())
            .collect::<Vec<_>>();
        for batch in batched {
            let client = self.client();
            let ck = self.ck();
            let cs = Some(self.cs());
            let url = uri.clone();
            set.spawn(async move {
                client
                    .post(url)
                    .basic_auth(ck, cs)
                    .json(&batch)
                    .send()
                    .await?
                    .json::<BatchObject<T>>()
                    .await
            });
        }
        while let Some(Ok(Ok(v))) = set.join_next().await {
            result.extend(v.create)
        }
        Ok(result.into_iter().flatten().collect::<Vec<_>>())
    }
    pub async fn batch_update<T: Entity, O: Serialize + Clone + Send + 'static>(
        &self,
        update_objects: Vec<O>,
    ) -> Result<Vec<T>> {
        let mut result = Vec::new();
        let mut set = JoinSet::new();
        let uri = self.base_url.join(&T::endpoint())?.join(BATCH)?;
        let batched = update_objects
            .chunks(100)
            .map(|c| BatchObject::builder().extend_update(c.to_vec()).build())
            .collect::<Vec<_>>();
        for batch in batched {
            let client = self.client();
            let ck = self.ck();
            let cs = Some(self.cs());
            let url = uri.clone();
            set.spawn(async move {
                client
                    .post(url)
                    .basic_auth(ck, cs)
                    .json(&batch)
                    .send()
                    .await?
                    .json::<BatchObject<T>>()
                    .await
            });
        }
        while let Some(Ok(Ok(v))) = set.join_next().await {
            result.extend(v.update)
        }
        Ok(result.into_iter().flatten().collect::<Vec<_>>())
    }
    pub async fn batch_delete<T: Entity, O: Serialize + Clone + Send + 'static>(
        &self,
        delete_objects: Vec<O>,
    ) -> Result<Vec<T>> {
        let mut result = Vec::new();
        let mut set = JoinSet::new();
        let uri = self.base_url.join(&T::endpoint())?.join(BATCH)?;
        let batched = delete_objects
            .chunks(100)
            .map(|c| BatchObject::builder().extend_delete(c.to_vec()).build())
            .collect::<Vec<_>>();
        for batch in batched {
            let client = self.client();
            let ck = self.ck();
            let cs = Some(self.cs());
            let url = uri.clone();
            set.spawn(async move {
                client
                    .post(url)
                    .basic_auth(ck, cs)
                    .json(&batch)
                    .send()
                    .await?
                    .json::<BatchObject<T>>()
                    .await
            });
        }
        while let Some(Ok(Ok(v))) = set.join_next().await {
            result.extend(v.delete)
        }
        Ok(result.into_iter().flatten().collect::<Vec<_>>())
    }
    pub async fn retrieve_subentity<T: Entity>(
        &self,
        entity_id: i32,
        subentity_id: i32,
    ) -> Result<T> {
        let uri = self
            .base_url
            .join(&T::child_endpoint(entity_id))?
            .join(&subentity_id.to_string())?;
        for i in 1..3 {
            tracing::debug!("Connecting {uri}, try {i}");
            match self
                .client
                .get(uri.clone())
                .basic_auth(self.ck(), Some(self.cs()))
                .send()
                .await?
                .json::<T>()
                .await
            {
                Ok(r) => return Ok(r),
                Err(e) => {
                    tracing::error!(
                        "Failed to connect to {uri} with error: {e}\n{} tries left",
                        3 - i
                    );
                    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                    continue;
                }
            }
        }
        Err(anyhow!(
            "Error retrieving subentitity with id: {subentity_id}"
        ))
    }
    pub async fn list_all_subentities<T: Entity>(&self, entity_id: i32) -> Result<Vec<T>> {
        let uri = self.base_url.join(&T::child_endpoint(entity_id))?;
        for i in 1..3 {
            tracing::debug!("Connecting {uri}, try {i}");
            match self
                .client
                .get(uri.clone())
                .basic_auth(self.ck(), Some(self.cs()))
                .send()
                .await?
                .json::<Vec<T>>()
                .await
            {
                Ok(r) => return Ok(r),
                Err(e) => {
                    tracing::error!(
                        "Failed to connect to {uri} with error: {e}\n{} tries left",
                        3 - i
                    );
                    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                    continue;
                }
            }
        }
        Err(anyhow!(
            "Error retrieving subentitity for entity with id: {entity_id}"
        ))
    }
    pub async fn create_subentity<T: Entity>(
        &self,
        entity_id: i32,
        object: impl Serialize,
    ) -> Result<T> {
        let uri = self.base_url.join(&T::child_endpoint(entity_id))?;
        for i in 1..3 {
            tracing::debug!("Connecting {uri}, try {i}");
            match self
                .client
                .post(uri.clone())
                .basic_auth(self.ck(), Some(self.cs()))
                .json(&object)
                .send()
                .await?
                .json::<T>()
                .await
            {
                Ok(r) => {
                    return Ok(r);
                }
                Err(e) => {
                    tracing::error!(
                        "Failed to connect to {uri} with error: {e}\n{} tries left",
                        3 - i
                    );
                    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                    continue;
                }
            }
        }
        Err(anyhow!("Error creating subentitity for id: {entity_id}"))
    }
    pub async fn update_subentity<T: Entity>(
        &self,
        entity_id: i32,
        subentity_id: i32,
        object: impl Serialize,
    ) -> Result<T> {
        let uri = self
            .base_url
            .join(&T::child_endpoint(entity_id))?
            .join(&subentity_id.to_string())?;
        for i in 1..3 {
            tracing::debug!("Connecting {uri}, try {i}");
            match self
                .client
                .put(uri.clone())
                .basic_auth(self.ck(), Some(self.cs()))
                .json(&object)
                .send()
                .await?
                .json::<T>()
                .await
            {
                Ok(r) => return Ok(r),
                Err(e) => {
                    tracing::error!(
                        "Failed to connect to {uri} with error: {e}\n{} tries left",
                        3 - i
                    );
                    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                    continue;
                }
            }
        }
        Err(anyhow!(
            "Error updating subentitity with id: {subentity_id}"
        ))
    }
    pub async fn delete_subentity<T: Entity>(
        &self,
        entity_id: i32,
        subentity_id: i32,
    ) -> Result<T> {
        let uri = self
            .base_url
            .join(&T::child_endpoint(entity_id))?
            .join(&subentity_id.to_string())?;
        for i in 1..3 {
            tracing::debug!("Connecting {uri}, try {i}");
            match self
                .client
                .delete(uri.clone())
                .basic_auth(self.ck(), Some(self.cs()))
                .send()
                .await?
                .json::<T>()
                .await
            {
                Ok(r) => return Ok(r),
                Err(e) => {
                    tracing::error!(
                        "Failed to connect to {uri} with error: {e}\n{} tries left",
                        3 - i
                    );
                    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                    continue;
                }
            }
        }
        Err(anyhow!(
            "Error deleting subentitity with id: {subentity_id}"
        ))
    }
    pub async fn batch_create_subentity<T: Entity, O: Serialize + Clone + Send + 'static>(
        &self,
        entity_id: i32,
        create_objects: Vec<O>,
    ) -> Result<Vec<T>> {
        let mut result = Vec::new();
        let mut set = JoinSet::new();
        let uri = self
            .base_url
            .join(&T::child_endpoint(entity_id))?
            .join(BATCH)?;
        let batched = create_objects
            .chunks(100)
            .map(|c| BatchObject::builder().extend_create(c.to_vec()).build())
            .collect::<Vec<_>>();
        for batch in batched {
            let client = self.client();
            let ck = self.ck();
            let cs = Some(self.cs());
            let url = uri.clone();
            set.spawn(async move {
                client
                    .post(url)
                    .basic_auth(ck, cs)
                    .json(&batch)
                    .send()
                    .await?
                    .json::<BatchObject<T>>()
                    .await
            });
        }
        while let Some(Ok(Ok(v))) = set.join_next().await {
            result.extend(v.create)
        }
        Ok(result.into_iter().flatten().collect::<Vec<_>>())
    }
    pub async fn batch_update_subentity<T: Entity, O: Serialize + Clone + Send + 'static>(
        &self,
        entity_id: i32,
        update_objects: Vec<O>,
    ) -> Result<Vec<T>> {
        let mut result = Vec::new();
        let mut set = JoinSet::new();
        let uri = self
            .base_url
            .join(&T::child_endpoint(entity_id))?
            .join(BATCH)?;
        let batched = update_objects
            .chunks(100)
            .map(|c| BatchObject::builder().extend_update(c.to_vec()).build())
            .collect::<Vec<_>>();
        for batch in batched {
            let client = self.client();
            let ck = self.ck();
            let cs = Some(self.cs());
            let url = uri.clone();
            set.spawn(async move {
                client
                    .post(url)
                    .basic_auth(ck, cs)
                    .json(&batch)
                    .send()
                    .await?
                    .json::<BatchObject<T>>()
                    .await
            });
        }
        while let Some(Ok(Ok(v))) = set.join_next().await {
            result.extend(v.update)
        }
        Ok(result.into_iter().flatten().collect::<Vec<_>>())
    }
    pub async fn batch_delete_subentity<T: Entity, O: Serialize + Clone + Send + 'static>(
        &self,
        entity_id: i32,
        delete_objects: Vec<O>,
    ) -> Result<Vec<T>> {
        let mut result = Vec::new();
        let mut set = JoinSet::new();
        let uri = self
            .base_url
            .join(&T::child_endpoint(entity_id))?
            .join(BATCH)?;
        let batched = delete_objects
            .chunks(100)
            .map(|c| BatchObject::builder().extend_delete(c.to_vec()).build())
            .collect::<Vec<_>>();
        for batch in batched {
            let client = self.client();
            let ck = self.ck();
            let cs = Some(self.cs());
            let url = uri.clone();
            set.spawn(async move {
                client
                    .post(url)
                    .basic_auth(ck, cs)
                    .json(&batch)
                    .send()
                    .await?
                    .json::<BatchObject<T>>()
                    .await
            });
        }
        while let Some(Ok(Ok(v))) = set.join_next().await {
            result.extend(v.delete)
        }
        Ok(result.into_iter().flatten().collect::<Vec<_>>())
    }
}
