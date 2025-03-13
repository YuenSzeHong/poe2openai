-- 创建存储请求/回复内容的表
CREATE TABLE IF NOT EXISTS interactions (
    id serial PRIMARY KEY,
    model text NOT NULL, -- 模型名称
    prompt text NOT NULL, -- 请求模型的输入
    response text NOT NULL, -- 请求模型的输出
    created_at timestamp DEFAULT current_timestamp
);

-- 添加索引以优化查询
CREATE INDEX IF NOT EXISTS idx_request_id ON interactions (id);
