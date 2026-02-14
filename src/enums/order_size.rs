use crate::types::lots::Lots;
use crate::types::quantity::Quantity;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OrderSize<QI, LI, LSI> {
    Quantity(Quantity<QI>),
    Lots(Lots<LI, LSI>),
}
