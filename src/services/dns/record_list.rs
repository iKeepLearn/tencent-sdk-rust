use crate::core::Endpoint;
use serde::Deserialize;
use serde_json::{json, Value};
use std::borrow::Cow;

/// 记录列表元素
#[derive(Debug, Deserialize)]
pub struct RecordListItem {
    /// 记录ID
    #[serde(rename = "RecordId")]
    pub record_id: u64,
    
    /// 记录值
    #[serde(rename = "Value")]
    pub value: String,
    
    /// 记录状态，启用：ENABLE，暂停：DISABLE
    #[serde(rename = "Status")]
    pub status: String,
    
    /// 更新时间
    #[serde(rename = "UpdatedOn")]
    pub updated_on: String,
    
    /// 主机名
    #[serde(rename = "Name")]
    pub name: String,
    
    /// 记录线路
    #[serde(rename = "Line")]
    pub line: String,
    
    /// 线路ID
    #[serde(rename = "LineId")]
    pub line_id: String,
    
    /// 记录类型
    #[serde(rename = "Type")]
    pub record_type: String,
    
    /// 记录权重，用于负载均衡记录
    #[serde(rename = "Weight")]
    pub weight: Option<u32>,
    
    /// 记录监控状态，正常：OK，告警：WARN，宕机：DOWN，未设置监控或监控暂停则为空
    #[serde(rename = "MonitorStatus")]
    pub monitor_status: Option<String>,
    
    /// 记录备注说明
    #[serde(rename = "Remark")]
    pub remark: Option<String>,
    
    /// 记录缓存时间，单位：秒
    #[serde(rename = "TTL")]
    pub ttl: u32,
    
    /// MX值
    #[serde(rename = "MX")]
    pub mx: Option<u32>,
    
    /// 是否是默认的ns记录
    #[serde(rename = "DefaultNS")]
    pub default_ns: bool,
}

/// 查询记录列表的数量统计信息
#[derive(Debug, Deserialize)]
pub struct RecordCountInfo {
    /// 子域名数量
    #[serde(rename = "SubdomainCount")]
    pub subdomain_count: u32,
    
    /// 列表返回的记录数
    #[serde(rename = "ListCount")]
    pub list_count: u32,
    
    /// 总的记录数
    #[serde(rename = "TotalCount")]
    pub total_count: u32,
}

/// 获取域名DNS记录响应
#[derive(Debug, Deserialize)]
pub struct DomainRecordListResponse {
    #[serde(rename = "Response")]
    pub response: DomainRecordListResult,
}

/// 获取域名DNS记录结果
#[derive(Debug, Deserialize)]
pub struct DomainRecordListResult {
    /// 记录的数量统计信息
    #[serde(rename = "RecordCountInfo")]
    pub record_count_info: RecordCountInfo,
    
    /// 获取的记录列表
    #[serde(rename = "RecordList")]
    pub record_list: Vec<RecordListItem>,
    
    /// 唯一请求ID
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

/// 请求参数结构体 - 获取域名DNS记录
pub struct DomainRecordList<'a> {
    /// 域名
    pub domain: &'a str,
    
    /// 域名ID。参数 DomainId 优先级比参数 Domain 高，如果传递参数 DomainId 将忽略参数 Domain
    pub domain_id: Option<u64>,
    
    /// 解析记录的主机头，如果传了此参数，则只会返回此主机头对应的解析记录
    pub subdomain: Option<&'a str>,
    
    /// 获取某种类型的解析记录，如 A，CNAME，NS，AAAA，显性URL，隐性URL，CAA，SPF等
    pub record_type: Option<&'a str>,
    
    /// 获取某条线路名称的解析记录
    pub record_line: Option<&'a str>,
    
    /// 获取某个线路Id对应的解析记录，如果传RecordLineId，系统会忽略RecordLine参数
    pub record_line_id: Option<&'a str>,
    
    /// 获取某个分组下的解析记录时，传这个分组Id
    pub group_id: Option<u64>,
    
    /// 通过关键字搜索解析记录，当前支持搜索主机头和记录值
    pub keyword: Option<&'a str>,
    
    /// 排序字段，支持 name,line,type,value,weight,mx,ttl,updated_on 几个字段
    pub sort_field: Option<&'a str>,
    
    /// 排序方式，正序：ASC，逆序：DESC。默认值为ASC
    pub sort_type: Option<&'a str>,
    
    /// 偏移量，默认值为0
    pub offset: Option<u32>,
    
    /// 限制数量，当前Limit最大支持3000。默认值为100
    pub limit: Option<u32>,
}

impl<'a> DomainRecordList<'a> {
    /// 创建新的请求实例
    pub fn new(domain: &'a str) -> Self {
        Self {
            domain,
            domain_id: None,
            subdomain: None,
            record_type: None,
            record_line: None,
            record_line_id: None,
            group_id: None,
            keyword: None,
            sort_field: None,
            sort_type: None,
            offset: None,
            limit: None,
        }
    }
    
    /// 设置域名ID
    pub fn with_domain_id(mut self, domain_id: u64) -> Self {
        self.domain_id = Some(domain_id);
        self
    }
    
    /// 设置子域名（主机头）
    pub fn with_subdomain(mut self, subdomain: &'a str) -> Self {
        self.subdomain = Some(subdomain);
        self
    }
    
    /// 设置记录类型
    pub fn with_record_type(mut self, record_type: &'a str) -> Self {
        self.record_type = Some(record_type);
        self
    }
    
    /// 设置记录线路
    pub fn with_record_line(mut self, record_line: &'a str) -> Self {
        self.record_line = Some(record_line);
        self
    }
    
    /// 设置记录线路ID
    pub fn with_record_line_id(mut self, record_line_id: &'a str) -> Self {
        self.record_line_id = Some(record_line_id);
        self
    }
    
    /// 设置分组ID
    pub fn with_group_id(mut self, group_id: u64) -> Self {
        self.group_id = Some(group_id);
        self
    }
    
    /// 设置搜索关键字
    pub fn with_keyword(mut self, keyword: &'a str) -> Self {
        self.keyword = Some(keyword);
        self
    }
    
    /// 设置排序字段
    pub fn with_sort_field(mut self, sort_field: &'a str) -> Self {
        self.sort_field = Some(sort_field);
        self
    }
    
    /// 设置排序方式
    pub fn with_sort_type(mut self, sort_type: &'a str) -> Self {
        self.sort_type = Some(sort_type);
        self
    }
    
    /// 设置偏移量
    pub fn with_offset(mut self, offset: u32) -> Self {
        self.offset = Some(offset);
        self
    }
    
    /// 设置限制数量
    pub fn with_limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
}

impl<'a> Endpoint for DomainRecordList<'a> {
    type Output = DomainRecordListResponse;

    fn service(&self) -> Cow<'static, str> {
        Cow::Borrowed("dnspod")
    }

    fn action(&self) -> Cow<'static, str> {
        Cow::Borrowed("DescribeRecordList")
    }

    fn version(&self) -> Cow<'static, str> {
        Cow::Borrowed("2021-03-23")
    }

    fn region(&self) -> Option<Cow<'_, str>> {
        // DNS接口不需要region参数
        None
    }

    fn payload(&self) -> Value {
        let mut payload = json!({
            "Domain": self.domain,
        });

        if let Some(domain_id) = self.domain_id {
            payload["DomainId"] = json!(domain_id);
        }
        if let Some(subdomain) = self.subdomain {
            payload["Subdomain"] = json!(subdomain);
        }
        if let Some(record_type) = self.record_type {
            payload["RecordType"] = json!(record_type);
        }
        if let Some(record_line) = self.record_line {
            payload["RecordLine"] = json!(record_line);
        }
        if let Some(record_line_id) = self.record_line_id {
            payload["RecordLineId"] = json!(record_line_id);
        }
        if let Some(group_id) = self.group_id {
            payload["GroupId"] = json!(group_id);
        }
        if let Some(keyword) = self.keyword {
            payload["Keyword"] = json!(keyword);
        }
        if let Some(sort_field) = self.sort_field {
            payload["SortField"] = json!(sort_field);
        }
        if let Some(sort_type) = self.sort_type {
            payload["SortType"] = json!(sort_type);
        }
        if let Some(offset) = self.offset {
            payload["Offset"] = json!(offset);
        }
        if let Some(limit) = self.limit {
            payload["Limit"] = json!(limit);
        }

        payload
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_describe_record_list_builder() {
        let request = DomainRecordList::new("example.com")
            .with_domain_id(81345824)
            .with_subdomain("www")
            .with_record_type("A")
            .with_record_line("默认")
            .with_offset(0)
            .with_limit(100);

        assert_eq!(request.domain, "example.com");
        assert_eq!(request.domain_id, Some(81345824));
        assert_eq!(request.subdomain, Some("www"));
        assert_eq!(request.record_type, Some("A"));
        assert_eq!(request.record_line, Some("默认"));
        assert_eq!(request.offset, Some(0));
        assert_eq!(request.limit, Some(100));
    }

    #[test]
    fn test_describe_record_list_payload() {
        let request = DomainRecordList::new("dnspod.cn")
            .with_subdomain("www")
            .with_record_type("A")
            .with_keyword("book")
            .with_sort_field("type")
            .with_sort_type("asc")
            .with_offset(0)
            .with_limit(100);

        let payload = request.payload();
        assert_eq!(payload["Domain"], json!("dnspod.cn"));
        assert_eq!(payload["Subdomain"], json!("www"));
        assert_eq!(payload["RecordType"], json!("A"));
        assert_eq!(payload["Keyword"], json!("book"));
        assert_eq!(payload["SortField"], json!("type"));
        assert_eq!(payload["SortType"], json!("asc"));
        assert_eq!(payload["Offset"], json!(0));
        assert_eq!(payload["Limit"], json!(100));
    }

    #[test]
    fn test_deserialize_record_list_response() {
        let json = r#"{
            "Response": {
                "RecordCountInfo": {
                    "SubdomainCount": 2,
                    "ListCount": 2,
                    "TotalCount": 10
                },
                "RecordList": [
                    {
                        "RecordId": 1,
                        "Value": "1.1.1.1",
                        "Status": "ENABLE",
                        "UpdatedOn": "2021-03-28 11:27:09",
                        "Name": "m",
                        "Line": "默认",
                        "LineId": "0",
                        "Type": "A",
                        "Weight": 20,
                        "MonitorStatus": "OK",
                        "Remark": "用于api",
                        "TTL": 600,
                        "MX": 10,
                        "DefaultNS": true
                    },
                    {
                        "RecordId": 2,
                        "Value": "2.2.2.2",
                        "Status": "ENABLE",
                        "UpdatedOn": "2021-03-28 11:27:10",
                        "Name": "www",
                        "Line": "默认",
                        "LineId": "0",
                        "Type": "A",
                        "TTL": 600,
                        "DefaultNS": false
                    }
                ],
                "RequestId": "req-123456"
            }
        }"#;

        let response: DomainRecordListResponse = serde_json::from_str(json).unwrap();
        
        assert_eq!(response.response.record_count_info.subdomain_count, 2);
        assert_eq!(response.response.record_count_info.list_count, 2);
        assert_eq!(response.response.record_count_info.total_count, 10);
        assert_eq!(response.response.request_id, "req-123456");
        
        let records = &response.response.record_list;
        assert_eq!(records.len(), 2);
        
        let first_record = &records[0];
        assert_eq!(first_record.record_id, 1);
        assert_eq!(first_record.value, "1.1.1.1");
        assert_eq!(first_record.status, "ENABLE");
        assert_eq!(first_record.name, "m");
        assert_eq!(first_record.record_type, "A");
        assert_eq!(first_record.ttl, 600);
        assert_eq!(first_record.weight, Some(20));
        assert_eq!(first_record.monitor_status.as_deref(), Some("OK"));
        assert_eq!(first_record.remark.as_deref(), Some("用于api"));
        assert_eq!(first_record.mx, Some(10));
        assert_eq!(first_record.default_ns, true);
        
        let second_record = &records[1];
        assert_eq!(second_record.record_id, 2);
        assert_eq!(second_record.weight, None);
        assert_eq!(second_record.monitor_status, None);
        assert_eq!(second_record.remark, None);
        assert_eq!(second_record.mx, None);
    }

    #[test]
    fn test_endpoint_implementation() {
        let request = DomainRecordList::new("test.com");
        assert_eq!(request.service().as_ref(), "dnspod");
        assert_eq!(request.action().as_ref(), "DomainRecordList");
        assert_eq!(request.version().as_ref(), "2021-03-23");
        assert!(request.region().is_none());
    }

    #[test]
    fn test_deserialize_minimal_record() {
        let json = r#"{
            "Response": {
                "RecordCountInfo": {
                    "SubdomainCount": 1,
                    "ListCount": 1,
                    "TotalCount": 1
                },
                "RecordList": [
                    {
                        "RecordId": 123,
                        "Value": "example.com",
                        "Status": "ENABLE",
                        "UpdatedOn": "2021-03-28 11:27:09",
                        "Name": "@",
                        "Line": "默认",
                        "LineId": "0",
                        "Type": "CNAME",
                        "TTL": 600,
                        "DefaultNS": false
                    }
                ],
                "RequestId": "req-789012"
            }
        }"#;

        let response: DomainRecordListResponse = serde_json::from_str(json).unwrap();
        let record = &response.response.record_list[0];
        
        assert_eq!(record.record_id, 123);
        assert_eq!(record.record_type, "CNAME");
        assert_eq!(record.name, "@");
        assert_eq!(record.ttl, 600);
        assert_eq!(record.default_ns, false);
        assert_eq!(record.weight, None);
        assert_eq!(record.mx, None);
    }
}