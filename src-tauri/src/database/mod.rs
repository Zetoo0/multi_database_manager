pub trait Database{
    type Connection;

    fn execute_query(&self, sql:&str);
    fn begin_transaction(&self);
    fn commit_transaction(&self, conn:Self::Connection);
}