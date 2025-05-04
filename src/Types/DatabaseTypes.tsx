export type Entity<T> = {
    name:string;
    type:T;
  };
  
  export type Column = {
    name: string;
   // type: 'column';
    data_type: string;
    is_nullable: boolean;
    default_value: string;
    is_primary_key: boolean;
    maximum_length?: number;
    schema_name:string;
    table_name:string;
    db_name:string;
    type_:string;
  };
  
  export type Table = {
    name: string;
    type_: string;
    db_name:string;
    schema_name: string;
    columns: Record<string, Column>;
    indexes: Record<string, Index>;
    constraints: Record<string, Constraint>;
    rls_policies: Record<string, RlsPolicy>;
    rules: Record<string, Rule>;
    triggers: Record<string, Trigger>;
  };
  
  type RlsPolicy = {
    name: string;
    type: 'rls_policy';
    command: string;
  };
  
  type Rule = {
    name: string;
    type: 'rule';
    definition: string;
  };
  
  export type Trigger = {
    name: string;
    type: 'trigger';
    definition: string;
    type_:string;
  };
  
  export type TriggerFunction = {
    name: string;
    type: 'triggerfunction';
    definition: string;
    function_name: string;
    schema_name:string;
    type_:string;
  };

  export type Index = {
    name: string;
    type: 'index';
    definition: string | null;
    column_name: string[];
    non_unique: boolean;
    table_name: string;
    db_name:string;
    schema_name:string;
    type_:string;
  };
  
  export type Constraint = {
    name: string;
    type: 'constraint';
    type_:string;
    table_name: string;
    column_name: string;
    c_type: string;
    db_name:string;
    schema_name:string;
  };
  //[2025-01-14T14:53:21.395680 INFO  writepad_lib::domain::repository::postgres_repository] Constraints::{"rental_customer_id_fkey": Constraint { name: "rental_customer_id_fkey", c_type: "FOREIGN KEY", table_name: "rental", column_name: "customer_id", db_name: "dvdrental" }, "rental_staff_id_key": Constraint { name: "rental_staff_id_key", c_type: "FOREIGN KEY", table_name: "rental", column_name: "staff_id", db_name: "dvdrental" }, "rental_pkey": Constraint { name: "rental_pkey", c_type: "PRIMARY KEY", table_name: "rental", column_name: "rental_id", db_name: "dvdrental" }, "rental_inventory_id_fkey": Constraint { name: "rental_inventory_id_fkey", c_type: "FOREIGN KEY", table_name: "rental", column_name: "inventory_id", db_name: "dvdrental" }}

  export type Sequence = {
    name: string;
    type: 'sequence';
    start_val: string;
    minimum_val: string;
    maximum_val: string;
    increment: string;
    cycle: string;
    db_name:string;
    schema_name:string;
  };
  
  export type Functions = {
    name: string;
    type: 'function';
    definition: string;
    parameters: string[] | null;
    return_type: string;
    db_name:string;
    full_function:string;
    schema_name:string;
    children: { name: string; type: 'parameter' }[];
  };

  export type Function = {
    name: string;
    type: 'function';
    definition: string;
    return_type: string;
    db_name:string;
    schema_name:string;
    full_function:string;
    parameters: string[] | null;
  }

  export type Procedure = {
    name: string;
    type: 'procedure';
    definition: string;
    parameters: string[] | null;
    db_name:string;
    schema_name:string;
    children: { name: string; type: 'parameter' }[];
  };
  
  export type Schema = {
    name: string;
    type: "schema";
    functions: Record<string, Functions>;
    trigger_functions: Record<string, TriggerFunction>;
    procedures: Record<string, Procedure>;
    tables: Record<string, Table>;
    views: Record<string, View>;
    aggregates: Record<string, Aggregate>;
    constraints: Record<string, Constraint>;
    locks: Record<string, Lock>;
    materalized_views: Record<string, MateralizedView>;
    sequences: Record<string, Sequence>;
    catalogs: Record<string, Catalog>;
    roles: Record<string, Role>;
    types: Record<string, Type>;
    schema_name: string;
  };

  export type Role = {
    name: string;
    type: 'role';
    is_super? : boolean;
    is_insherit? : boolean;
    is_create_role? : boolean;
    is_create_db? : boolean;
    can_login? : boolean;
    is_replication? : boolean;
    connection_limit? : number;
    password? : string;
    valid_until? : string;
    db_name:string;
    schema_name:string;
    type_:string;
  }
  
  type Type = {
    name: string;
    type: 'type';
    fields: Record<string, TypeField>;
  }
  
  type MateralizedView = {
    name: string;
    type: 'materalized_view';
    definition: string;
  }
  
  type TypeField = {
    name: string;
    type_name: string;
  }
  
  export type View = {
    name: string;
    type: 'view';
    definition: string;
    schema_name: string;
    type_:string;
  };
  
  type Aggregate = {
    name: string;
    type: 'aggregate';
  }
  
  type Catalog = {
    name: string;
    type: "catalog";
    functions: Record<string, Functions>;
    procedures: Record<string, Procedure>;
    tables: Record<string, Table>;
    views: Record<string, any>;
  };
  
  type ForeignDataWrapper = {
    name: string;
    type: "foreign_data_wrapper";
    children: Record<string, any>;
  };
  
  export type Database = {
      name: string;
      type_:string;
      catalogs: Record<string, Catalog>;
      foreign_data_wrappers: Record<string, ForeignDataWrapper>;
      schemas: Record<string, Schema>;
  }