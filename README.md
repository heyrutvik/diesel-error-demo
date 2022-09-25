# diesel-error-demo

"Extended" output of the program.

I made [changes in "diesel"](https://github.com/heyrutvik/diesel/commit/1a8e2d84a672d3d8f5bee4a217cbc896314950e8) to understand and check connection health and errors.

Make sure you already have the same record in db so that it hits the fetch query.

We can see that before sending the second command, connection health was "OK".

```
------------------------------- establish -------------------------------
ExecuteDsl
+ execute_returning_count: AnsiTransactionManager { status: Valid(ValidTransactionManagerStatus { in_transaction: None }) }
++ execute = {self.name: Statement { name: "", param_formats: [] }, param_data: []}
+++ send_query_prepared = using connection [3535083226] for PQsendQueryPrepared.
==== cleaning connection ====
ExecuteDsl
+ execute_returning_count: AnsiTransactionManager { status: Valid(ValidTransactionManagerStatus { in_transaction: None }) }
++ execute = {self.name: Statement { name: "", param_formats: [] }, param_data: []}
+++ send_query_prepared = using connection [3535083226] for PQsendQueryPrepared.
==== cleaning connection ====
+ load
++ execute = {self.name: Statement { name: "", param_formats: [1, 1] }, param_data: ["name_1", "\0\0 o"]}
+++ send_query_prepared = using connection [3535083226] for PQsendQueryPrepared.
+ load
++ execute = {self.name: Statement { name: "__diesel_stmt_0", param_formats: [1, 1] }, param_data: ["name_1", "\0\0\0\0\0\0\0\u{1}"]}
+++ send_query_prepared = using connection [3535083226] for PQsendQueryPrepared.
2.Result: User { id: 40, name: "name_1", birth_date: 2022-09-25 }
+ load
++ execute = {self.name: Statement { name: "", param_formats: [1, 1] }, param_data: ["name_1", "\0\0 o"]}
+++ send_query_prepared = using connection [3535083226] for PQsendQueryPrepared.
+ load
++ execute = {self.name: Statement { name: "__diesel_stmt_0", param_formats: [1, 1] }, param_data: ["name_1", "\0\0\0\0\0\0\0\u{1}"]}
+++ send_query_prepared = using connection [3535083226] for PQsendQueryPrepared.
2.Error: 2.[CONNECTION_OK] another command is already in progress
## diesel::drop [3535083226] ##
```
As per my understanding: one of the reasons "another command is already in progress" being thrown is that when we don't clear connection by repeatedly getting results till we get NULL.

I added code to perform the same right before we call `PQsendQueryPrepared` and the following code will start working as it is.

See, https://github.com/heyrutvik/diesel/commit/1a8e2d84a672d3d8f5bee4a217cbc896314950e8#diff-d909dcd86a6b0f5a6c0a11dd876d0fa30e4b2baf340398847141b9e2b160599eR84

```
// UNCOMMENT FOLLOWING LINE TO GET IT WORKING!
// while self.get_next_result()?.is_some() {}
```
in diesel/src/pg/connection/raw.rs FILE.
