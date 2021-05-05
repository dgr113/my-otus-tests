#!/bin/bash

set -e
set -u


function create_notify_db() {
	echo "Creating 'notify' database ..."
	psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" <<-EOSQL
	    CREATE DATABASE notify;
	    GRANT ALL PRIVILEGES ON DATABASE notify TO "$POSTGRES_USER";

	    \connect notify

      CREATE TABLE notifications (
          id                  SERIAL primary key,
          user_id             INTEGER not null,
          account_id          INTEGER not null,
          available           REAL,
          spent               REAL,
          was_succeed         BOOLEAN not null,
          date                TIMESTAMPTZ not null
      );
EOSQL
}

function create_billing_db() {
	echo "Creating 'billing' database ..."
	psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" <<-EOSQL
	    CREATE DATABASE billing;
	    GRANT ALL PRIVILEGES ON DATABASE billing TO "$POSTGRES_USER";

	    \connect billing

	    CREATE TABLE accounts (
          id                  SERIAL primary key,
          user_id             INTEGER not null,
          user_name           TEXT not null,
          available           REAL,
          spent               REAL,
          create_date         TIMESTAMPTZ not null,
          update_date         TIMESTAMPTZ not null
      );
EOSQL
}

function create_orders_db() {
	echo "Creating 'orders' database ..."
	psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" <<-EOSQL
	    CREATE DATABASE orders;
	    GRANT ALL PRIVILEGES ON DATABASE orders TO "$POSTGRES_USER";

	    \connect orders

      CREATE TABLE orders (
          id                  SERIAL primary key,
          create_date         TIMESTAMPTZ not null,
          update_date         TIMESTAMPTZ not null,
          customer_id         INTEGER not null,
          status_code         INTEGER not null
      );

      CREATE TABLE order_products (
          id                  SERIAL primary key,
          order_id            INTEGER not null references orders (id) on delete cascade,
          price               REAL not null,
          quantity            INTEGER not null,
          product_name        TEXT not null,
          product_id          INTEGER
      );

      CREATE TABLE order_unique_keys (
          id                  SERIAL primary key,
          order_id            INTEGER not null references orders (id) on delete cascade,
          unique_key          TEXT not null unique
      );
EOSQL
}


echo "Starting of INITDB script..."
create_orders_db
create_billing_db
create_notify_db
echo "End of INITDB script."
