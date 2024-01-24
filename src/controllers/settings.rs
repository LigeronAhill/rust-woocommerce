use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingOptionUpdate {
    /// Setting value.
    pub value: serde_json::Value,
}
#[cfg(test)]
mod tests {
    use crate::{
        setting_options::SettingOption, settings::SettingGroup, ApiClient, Entity, SubEntity,
    };

    #[tokio::test]
    async fn test_update_setting_options() {
        let client = ApiClient::from_env().unwrap();
        let gr_id = "products";
        let opt_id = "woocommerce_review_rating_verification_label";
        let upd = SettingOption::update("no");
        let _updated: SettingOption = client
            .update_subentity(
                Entity::Setting,
                gr_id,
                SubEntity::SettingOption,
                opt_id,
                upd,
            )
            .await
            .unwrap();
        let upd = SettingOption::update("yes");
        let updated: SettingOption = client
            .update_subentity(
                Entity::Setting,
                gr_id,
                SubEntity::SettingOption,
                opt_id,
                upd,
            )
            .await
            .unwrap();
        assert_eq!(updated.id, opt_id);
    }
    #[tokio::test]
    async fn test_list_all_settings_list_all_aetting_options_retrieve_setting_option() {
        let client = ApiClient::from_env().unwrap();
        let result = client
            .list_all::<SettingGroup>(Entity::Setting)
            .await
            .unwrap();
        assert!(!result.is_empty());
        if let Some(group) = result.first() {
            let opt: Vec<SettingOption> = client
                .retrieve(Entity::Setting, group.id.to_owned())
                .await
                .unwrap();
            assert!(!opt.is_empty());
            if let Some(first_opt) = opt.first() {
                let option: SettingOption = client
                    .retrieve_subentity(
                        Entity::Setting,
                        group.id.to_owned(),
                        SubEntity::SettingOption,
                        first_opt.id.to_owned(),
                    )
                    .await
                    .unwrap();
                assert_eq!(option.id, first_opt.id);
            }
        }
    }
}
