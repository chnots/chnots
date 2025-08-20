use chin_tools::EResult;

use crate::krate::toent::po::ToentTimeEventInst;

pub(crate) trait ToentMapper {
    async fn insert_toent_time_event_inst(inst: ToentTimeEventInst) -> EResult;
}
