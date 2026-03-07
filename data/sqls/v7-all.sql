CREATE TABLE
    toent_todo (
        otid BIGINT NOT NULL,
        todo_state VARCHAR(30),
        todo_priority BIGINT,
        alert_tid BIGINT,
        start_tid BIGINT,
        end_tid BIGINT,
        timezone BIGINT,
        closed BOOLEAN,
        tid BIGINT NOT NULL,
        note TEXT,
        PRIMARY KEY (otid),
        UNIQUE (tid)
    );

CREATE TABLE
    toent_todo_hist (
        otid BIGINT NOT NULL,
        todo_state VARCHAR(30),
        todo_priority BIGINT,
        alert_tid BIGINT,
        start_tid BIGINT,
        end_tid BIGINT,
        timezone BIGINT,
        closed BOOLEAN,
        tid BIGINT NOT NULL,
        note TEXT,
        UNIQUE (tid)
    );

CREATE TABLE
    toent_event (
        otid BIGINT NOT NULL,
        event_defi TEXT NOT NULL,
        start_time BIGINT,
        start_timezone BIGINT,
        end_time BIGINT,
        end_timezone BIGINT,
        tid BIGINT NOT NULL,
        PRIMARY KEY (otid),
        UNIQUE (tid)
    );

CREATE TABLE
    toent_event_hist (
        otid BIGINT NOT NULL,
        event_defi TEXT NOT NULL,
        start_time BIGINT,
        start_timezone BIGINT,
        end_time BIGINT,
        end_timezone BIGINT,
        tid BIGINT NOT NULL,
        UNIQUE (tid)
    );
