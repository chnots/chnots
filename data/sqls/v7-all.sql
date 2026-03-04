CREATE TABLE IF NOT EXISTS toent_todo (
    otid BIGINT NOT NULL,
    todo_priority BIGINT,
    todo_state VARCHAR(30),
    todo_closed BOOLEAN NOT NULL,
    tid BIGINT NOT NULL,
    PRIMARY KEY (otid),
    UNIQUE (tid)
);

CREATE TABLE IF NOT EXISTS toent_todo_hist (
    otid BIGINT NOT NULL,
    todo_priority BIGINT,
    todo_state VARCHAR(30),
    todo_closed BOOLEAN NOT NULL,
    tid BIGINT NOT NULL,
    UNIQUE (tid)
);

CREATE TABLE IF NOT EXISTS toent_event (
    otid BIGINT NOT NULL,
    event_defi TEXT NOT NULL,
    tid BIGINT NOT NULL,
    PRIMARY KEY (otid),
    UNIQUE (tid)
);

CREATE TABLE IF NOT EXISTS toent_event_hist (
    otid BIGINT NOT NULL,
    event_defi TEXT NOT NULL,
    tid BIGINT NOT NULL,
    UNIQUE (tid)
);

CREATE TABLE IF NOT EXISTS todo_inst (
    otid BIGINT NOT NULL,
    timezone VARCHAR(10),
    naive_time VARCHAR(30) NOT NULL,
    target_status VARCHAR(30),
    note TEXT,
    alert_tid BIGINT,
    target_tid BIGINT NOT NULL,
    tid BIGINT NOT NULL,
    PRIMARY KEY (otid, target_tid),
    UNIQUE (tid)
);

CREATE TABLE IF NOT EXISTS todo_inst_hist (
    otid BIGINT NOT NULL,
    timezone VARCHAR(10),
    naive_time VARCHAR(30) NOT NULL,
    target_status VARCHAR(30),
    note TEXT,
    alert_tid BIGINT,
    target_tid BIGINT NOT NULL,
    tid BIGINT NOT NULL,
    UNIQUE (tid)
);
