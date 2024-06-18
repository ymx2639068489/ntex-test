// @generated automatically by Diesel CLI.

diesel::table! {
    admin (id) {
        #[max_length = 36]
        id -> Varchar,
        #[max_length = 36]
        role_id -> Varchar,
        #[max_length = 36]
        company_id -> Varchar,
        #[max_length = 20]
        username -> Varchar,
        #[max_length = 100]
        password -> Varchar,
        #[max_length = 20]
        nickname -> Varchar,
    }
}

diesel::table! {
    base_product (id) {
        #[max_length = 36]
        id -> Varchar,
        create_at -> Datetime,
        #[sql_name = "type"]
        #[max_length = 50]
        type_ -> Varchar,
        #[max_length = 50]
        name -> Varchar,
        #[max_length = 500]
        file_list -> Nullable<Varchar>,
        #[max_length = 500]
        notes -> Nullable<Varchar>,
    }
}

diesel::table! {
    company (id) {
        #[max_length = 36]
        id -> Varchar,
        #[max_length = 20]
        name -> Varchar,
    }
}

diesel::table! {
    custom (id) {
        id -> Integer,
        #[max_length = 10]
        name -> Varchar,
        #[max_length = 20]
        phone -> Varchar,
        #[max_length = 20]
        id_type -> Nullable<Varchar>,
        #[max_length = 50]
        id_number -> Nullable<Varchar>,
        level -> Integer,
    }
}

diesel::table! {
    custom_salesman (id) {
        id -> Integer,
        custom_id -> Integer,
        salesman_id -> Integer,
        #[max_length = 36]
        product_id -> Varchar,
        create_at -> Datetime,
        #[max_length = 20]
        company -> Varchar,
        #[max_length = 30]
        order_id -> Varchar,
        #[max_length = 30]
        pay_method -> Varchar,
        money -> Decimal,
        people_number -> Integer,
        #[max_length = 50]
        rebate -> Nullable<Varchar>,
    }
}

diesel::table! {
    ledger (id) {
        #[max_length = 36]
        id -> Varchar,
        #[max_length = 36]
        product_id -> Varchar,
        #[max_length = 50]
        product_name -> Varchar,
        start_time -> Datetime,
        end_time -> Datetime,
        people_number -> Integer,
        #[max_length = 20]
        product_type -> Varchar,
        duration -> Integer,
        revenue -> Decimal,
        cost -> Decimal,
        #[max_length = 20]
        pay_status -> Varchar,
        #[max_length = 20]
        executor -> Varchar,
        #[max_length = 500]
        notes -> Nullable<Varchar>,
    }
}

diesel::table! {
    operator (id) {
        id -> Integer,
        #[max_length = 36]
        admin_id -> Varchar,
        #[max_length = 30]
        teablename -> Varchar,
        #[max_length = 32]
        source_id -> Varchar,
        created_at -> Datetime,
        #[max_length = 20]
        operator_type -> Varchar,
        #[max_length = 4000]
        origin_object -> Nullable<Varchar>,
        #[max_length = 4000]
        now_object -> Nullable<Varchar>,
        #[max_length = 300]
        notes -> Varchar,
    }
}

diesel::table! {
    product (id) {
        #[max_length = 36]
        id -> Varchar,
        #[max_length = 36]
        base_product_id -> Varchar,
        create_at -> Datetime,
        start_time -> Datetime,
        end_time -> Datetime,
        people_number -> Integer,
        duration -> Integer,
        #[max_length = 500]
        notes -> Nullable<Varchar>,
    }
}

diesel::table! {
    role (id) {
        #[max_length = 36]
        id -> Varchar,
        #[max_length = 20]
        rolename -> Varchar,
        #[max_length = 100]
        description -> Varchar,
        #[max_length = 4000]
        router -> Varchar,
        admin_value -> Integer,
        operator_value -> Integer,
        role_value -> Integer,
        company_value -> Integer,
        salesman_value -> Integer,
        sales_records_value -> Integer,
        product_value -> Integer,
        custom_value -> Integer,
        base_product_value -> Integer,
        ledger_value -> Integer,
    }
}

diesel::table! {
    salesman (id) {
        id -> Integer,
        #[max_length = 36]
        company_id -> Varchar,
        #[max_length = 20]
        username -> Varchar,
        #[max_length = 20]
        phone -> Varchar,
    }
}

diesel::joinable!(admin -> company (company_id));
diesel::joinable!(admin -> role (role_id));
diesel::joinable!(custom_salesman -> custom (custom_id));
diesel::joinable!(custom_salesman -> product (product_id));
diesel::joinable!(custom_salesman -> salesman (salesman_id));
diesel::joinable!(operator -> admin (admin_id));
diesel::joinable!(product -> base_product (base_product_id));
diesel::joinable!(salesman -> company (company_id));

diesel::allow_tables_to_appear_in_same_query!(
    admin,
    base_product,
    company,
    custom,
    custom_salesman,
    ledger,
    operator,
    product,
    role,
    salesman,
);
