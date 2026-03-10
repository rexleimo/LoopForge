use rexos::config::{AwsBedrockConfig, ProviderKind, RexosConfig};

use super::super::super::super::{CheckStatus, DoctorCheck};

pub(super) fn push_router_status(checks: &mut Vec<DoctorCheck>, cfg: &RexosConfig) {
    let mut bedrock_routes: Vec<&str> = Vec::new();
    for (kind, route) in [
        ("planning", &cfg.router.planning),
        ("coding", &cfg.router.coding),
        ("summary", &cfg.router.summary),
    ] {
        let id = format!("router.{kind}.provider");
        if let Some(provider) = cfg.providers.get(&route.provider) {
            checks.push(DoctorCheck {
                id,
                status: CheckStatus::Ok,
                message: route.provider.clone(),
            });

            if provider.kind == ProviderKind::Bedrock {
                bedrock_routes.push(kind);
                push_bedrock_route_checks(checks, kind, &route.provider, cfg, route.model.as_str());
            }
        } else {
            checks.push(DoctorCheck {
                id,
                status: CheckStatus::Error,
                message: format!(
                    "unknown provider '{}' (defined: [{}])",
                    route.provider,
                    cfg.providers.keys().cloned().collect::<Vec<_>>().join(", ")
                ),
            });
        }
    }

    if !bedrock_routes.is_empty() {
        let routes = bedrock_routes.join(", ");
        let compiled = cfg!(feature = "bedrock");
        checks.push(DoctorCheck {
            id: "bedrock.feature".to_string(),
            status: if compiled {
                CheckStatus::Ok
            } else {
                CheckStatus::Error
            },
            message: if compiled {
                format!("compiled with bedrock support (routes: {routes})")
            } else {
                format!(
                    "bedrock routing configured (routes: {routes}) but this binary was built without bedrock support; rebuild with `cargo build -p loopforge-cli --features bedrock`"
                )
            },
        });
    }
}

fn push_bedrock_route_checks(
    checks: &mut Vec<DoctorCheck>,
    kind: &str,
    provider_name: &str,
    cfg: &RexosConfig,
    route_model: &str,
) {
    let provider = match cfg.providers.get(provider_name) {
        Some(provider) => provider,
        None => return,
    };

    let model_id = format!("bedrock.router.{kind}.model");
    let route_model = route_model.trim();
    if route_model.is_empty() {
        checks.push(DoctorCheck {
            id: model_id,
            status: CheckStatus::Error,
            message: format!(
                "router.{kind}.model is empty; set it to \"default\" or an explicit Bedrock model id"
            ),
        });
    } else if route_model == "default" && provider.default_model.trim().is_empty() {
        checks.push(DoctorCheck {
            id: model_id,
            status: CheckStatus::Error,
            message: format!(
                "router.{kind}.model = \"default\" but providers.{provider_name}.default_model is empty"
            ),
        });
    } else {
        checks.push(DoctorCheck {
            id: model_id,
            status: CheckStatus::Ok,
            message: if route_model == "default" {
                format!("default -> {}", provider.default_model.trim())
            } else {
                route_model.to_string()
            },
        });
    }

    let region_id = format!("bedrock.providers.{provider_name}.region");
    match provider.aws_bedrock.as_ref() {
        Some(aws) => {
            let region = aws.region.trim();
            if region.is_empty() {
                checks.push(DoctorCheck {
                    id: region_id,
                    status: CheckStatus::Error,
                    message: format!("providers.{provider_name}.aws_bedrock.region is empty"),
                });
            } else {
                checks.push(DoctorCheck {
                    id: region_id,
                    status: CheckStatus::Ok,
                    message: region.to_string(),
                });
            }
        }
        None => {
            let default_region = AwsBedrockConfig::default().region;
            checks.push(DoctorCheck {
                id: region_id,
                status: CheckStatus::Warn,
                message: format!(
                    "providers.{provider_name}.aws_bedrock is missing; defaulting region to {default_region}"
                ),
            });
        }
    }
}
