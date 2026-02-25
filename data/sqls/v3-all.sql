ALTER TABLE chnot_thread_order
ADD COLUMN closed BOOL default false;

ALTER TABLE chnot_thread_order_hist
ADD COLUMN closed BOOL default false;