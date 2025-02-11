// Copyright 2021 Datafuse Labs.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use common_meta_api::KVApi;
use common_meta_types::GetKVActionReply;
use common_meta_types::MGetKVActionReply;
use common_meta_types::PrefixListReply;
use common_meta_types::UpsertKVAction;
use common_meta_types::UpsertKVActionReply;

use crate::grpc_action::GetKVAction;
use crate::grpc_action::MGetKVAction;
use crate::grpc_action::PrefixListReq;
use crate::MetaGrpcClient;

#[tonic::async_trait]
impl KVApi for MetaGrpcClient {
    async fn upsert_kv(
        &self,
        act: UpsertKVAction,
    ) -> common_exception::Result<UpsertKVActionReply> {
        self.do_write(act).await
    }

    async fn get_kv(&self, key: &str) -> common_exception::Result<GetKVActionReply> {
        self.do_get(GetKVAction {
            key: key.to_string(),
        })
        .await
    }

    async fn mget_kv(&self, keys: &[String]) -> common_exception::Result<MGetKVActionReply> {
        let keys = keys.to_vec();
        self.do_get(MGetKVAction { keys }).await
    }

    async fn prefix_list_kv(&self, prefix: &str) -> common_exception::Result<PrefixListReply> {
        self.do_get(PrefixListReq(prefix.to_string())).await
    }
}
