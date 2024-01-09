// Api layer 定义了对外的接口和路由
pub mod api;
// App layer 定义了应用层的逻辑
pub mod application;
// Domain layer 定义了领域层的逻辑(模型和实体)
pub mod domain;
// Infastructure layer 定义了基础设施层的逻辑(数据库, 缓存, 消息队列...)
pub mod infastructure;
