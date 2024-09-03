//! using [`bon`](https://elastio.github.io/bon/) crate, which seems excellent
//!
//! great ergonomics for direct application to a struct
//! more importantly, it can create builders from functions.
//! and creates a struct builder from an `impl ... new()` by default
//! this allows single builder operation on these nested json structs
//! it's also a nice way of easily crating alternate builders
//! and for adding logic to builders (e.g. validation, etc.)

use bon::{bon, builder};
use derive_more::derive::From;
use serde::{Deserialize, Serialize};
use tracing::Level;

/// Note: this is *runtime* chekced.  Do **NOT** want
#[derive(Default, Debug)]
#[builder(on(String, into))]
#[allow(dead_code)]
pub struct ChannelBon {
        token:         i32,
        special_info:  u8,
        maybe_1:       Option<u32>,
        maybe_2:       Option<u32>,
        internal_data: String,
}

/// just here to play with builder syntax in-lib
#[allow(dead_code)]
fn chan_builder_bon() -> ChannelBon {
        // builder pattern, go, go, go!...
        let ch = ChannelBon::builder().token(13).special_info(5u8).maybe_2(133).internal_data("splat").build();
        println!("{:?}", ch);
        ch
}
/// just here to play with builder syntax in-lib
#[allow(dead_code)]
fn incident_builder_bon_fn() -> IncidentPostBon {
        let inc_bod = Incident::builder().title("title am I").service(String::from("serv")).urgency("fast_pls").build();
        IncidentPostBon { incident: inc_bod }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, From)]
pub struct IncidentPostBon {
        pub incident: Incident,
}
// NOTE: `#[bon]`
#[bon]
impl IncidentPostBon {
        /// Secret sauce: `fn new()` generates `Struct::builder()`
        #[builder(on(String, into))]
        pub fn new(title: String,
                   service: String,
                   priority: Option<String>,
                   incident_key: Option<String>,
                   urgency: Option<String>,
                   body: Option<String>,
                   ep: Option<(String, u32)>)
                   -> Self {
                let priority = priority.map(|id| PriorityReference { id });
                let body = body.map(|details| IncidentBody { details });
                let (escalation_policy, num) = match ep {
                        Some((id, n)) => (Some(EscalationPolicyReference { id }), n),
                        None => (None, 0),
                };
                tracing::event!(Level::INFO, num);
                let incident = Incident { title,
                                          service: ServiceReference { id: service },
                                          priority,
                                          incident_key,
                                          urgency,
                                          body,
                                          escalation_policy };
                IncidentPostBon { incident }
        }

        /// takes a single value, custom populates
        /// note that the chain is completed with `.call()` instead of `.build()`
        #[builder(finish_fn = "silly_build", on(String, into))]
        pub fn silly_string(title_theme: String) -> Self {
                let incident = Incident { title:             format!("silly, silly, here's our theme:{}-{}",
                                                                     title_theme, title_theme),
                                          service:           ServiceReference { id: String::from("B100P") },
                                          priority:          Some(PriorityReference { id: "0".to_string() }),
                                          incident_key:      None,
                                          urgency:           None,
                                          body:              Some(IncidentBody { details: "silly details".to_string(), }),
                                          escalation_policy: None, };
                IncidentPostBon { incident }
        }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, From)]
#[serde(tag = "type", rename = "incident")]
#[builder(on(String, into),on(Option<String>, into))]
pub struct Incident {
        pub title:   String,
        #[builder(into)]
        pub service: ServiceReference,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub priority:          Option<PriorityReference>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub urgency:           Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub incident_key:      Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub body:              Option<IncidentBody>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub escalation_policy: Option<EscalationPolicyReference>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, From)]
#[serde(tag = "type", rename = "service_reference")]
pub struct ServiceReference {
        pub id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, From)]
#[serde(tag = "type", rename = "priority_reference")]
pub struct PriorityReference {
        pub id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, From)]
#[serde(tag = "type", rename = "incident_body")]
pub struct IncidentBody {
        pub details: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, From)]
#[serde(tag = "type", rename = "escalation_policy_reference")]
pub struct EscalationPolicyReference {
        pub id: String,
}
