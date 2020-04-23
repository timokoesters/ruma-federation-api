//! [GET /_matrix/federation/v1/publicRooms](https://matrix.org/docs/spec/server_server/r0.1.3#get-matrix-federation-v1-publicrooms)

use js_int::UInt;
use ruma_api::ruma_api;
use ruma_identifiers::{RoomAliasId, RoomId};
use serde::{Deserialize, Serialize};

ruma_api! {
    metadata {
        description: "Gets all the public rooms for the homeserver.",
        method: GET,
        name: "get_public_rooms",
        path: "/_matrix/federation/v1/publicRooms",
        rate_limited: false,
        requires_authentication: true,
    }

    request {
        /// The maximum number of rooms to return. Default is no limit.
        #[serde(skip_serializing_if = "Option::is_none")]
        #[ruma_api(query)]
        pub limit: Option<UInt>,
        /// Pagination token from a previous request.
        #[serde(skip_serializing_if = "Option::is_none")]
        #[ruma_api(query)]
        pub since: Option<String>,
        /// Whether or not to include all networks/protocols defined by application services on the
        /// homeserver. Defaults to false.
        #[serde(skip_serializing_if = "Option::is_none")]
        #[ruma_api(query)]
        pub include_all_networks: Option<bool>,
        /// The specific third party network/protocol to request from the homeserver. Can only be used if include_all_networks is false.
        #[serde(skip_serializing_if = "Option::is_none")]
        #[ruma_api(query)]
        pub third_party_instance_id: Option<String>,
    }

    response {
        /// A paginated chunk of public rooms.
        pub chunk: Vec<PublicRoomsChunk>,
        /// A pagination token for the response.
        pub next_batch: Option<String>,
        /// A pagination token that allows fetching previous results.
        pub prev_batch: Option<String>,
        /// An estimate on the total number of public rooms, if the server has an estimate.
        pub total_room_count_estimate: Option<UInt>,
    }
}

/// A chunk of a room list response, describing one room
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PublicRoomsChunk {
    /// Aliases of the room.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub aliases: Vec<RoomAliasId>,
    /// The canonical alias of the room, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canonical_alias: Option<String>,
    /// The name of the room, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The number of members joined to the room.
    pub num_joined_members: UInt,
    /// The ID of the room.
    pub room_id: RoomId,
    /// The topic of the room, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
    /// Whether the room may be viewed by guest users without joining.
    pub world_readable: bool,
    /// Whether guest users may join the room and participate in it.
    ///
    /// If they can, they will be subject to ordinary power level rules like any other user.
    pub guest_can_join: bool,
    /// The URL for the room's avatar, if one is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
}
