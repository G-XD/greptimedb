searchState.loadedDescShard("table", 0, "Table and TableEngine requests\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nSNAFU context selector for the <code>Error::BuildColumnDescriptor</code>…\nSNAFU context selector for the <code>Error::ColumnExists</code> variant\nSNAFU context selector for the <code>Error::ColumnNotExists</code> …\nSNAFU context selector for the <code>Error::Datafusion</code> variant\nSNAFU context selector for the <code>Error::DuplicatedExecuteCall</code>…\nContains the error value\nDefault error implementation of table.\nSNAFU context selector for the <code>Error::InvalidAlterRequest</code> …\nSNAFU context selector for the <code>Error::InvalidTable</code> variant\nSNAFU context selector for the …\nContains the success value\nSNAFU context selector for the <code>Error::ParseTableOption</code> …\nSNAFU context selector for the <code>Error::RegionSchemaMismatch</code> …\nSNAFU context selector for the <code>Error::RemoveColumnInIndex</code> …\nSNAFU context selector for the <code>Error::SchemaBuild</code> variant\nSNAFU context selector for the <code>Error::SchemaConversion</code> …\nSNAFU context selector for the <code>Error::TableOperation</code> …\nSNAFU context selector for the <code>Error::TableProjection</code> …\nSNAFU context selector for the <code>Error::TablesRecordBatch</code> …\nSNAFU context selector for the <code>Error::Unsupported</code> variant\nConsume the selector and return the associated error\nConsume the selector and return the associated error\nConsume the selector and return the associated error\nConsume the selector and return the associated error\nConsume the selector and return the associated error\nConsume the selector and return the associated error\nConsume the selector and return the associated error\nConsume the selector and return the associated error\nConsume the selector and return the associated error\nConsume the selector and return the associated error\nConsume the selector and return a <code>Result</code> with the …\nConsume the selector and return a <code>Result</code> with the …\nConsume the selector and return a <code>Result</code> with the …\nConsume the selector and return a <code>Result</code> with the …\nConsume the selector and return a <code>Result</code> with the …\nConsume the selector and return a <code>Result</code> with the …\nConsume the selector and return a <code>Result</code> with the …\nConsume the selector and return a <code>Result</code> with the …\nConsume the selector and return a <code>Result</code> with the …\nConsume the selector and return a <code>Result</code> with the …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nAn ordinary physical table.\nContains the error value\nThe provider guarantees that all returned data satisfies …\nIndicates whether and how a filter expression can be …\nThe expression can be used to help minimise the data …\nContains the success value\nStruct used to serialize and deserialize <code>TableInfo</code>.\nStruct used to serialize and deserialize <code>TableMeta</code>.\nThe result after splitting requests by column location …\nIdentifier of the table.\nBuilder for <code>TableInfo</code>.\nError type for TableInfoBuilder\nThe table metadata Note: if you add new fields to this …\nBuilder for <code>TableMeta</code>.\nError type for TableMetaBuilder\nIndicates the type of this table for metadata/catalog …\nA transient table.\nUninitialized field\nUninitialized field\nThe expression cannot be used by the provider.\nCustom validation error\nCustom validation error\nA non-materialised table that itself uses a query …\nAllocate a new column for the table.\nBuilds a new <code>TableMeta</code>.\nBuilds a new <code>TableInfo</code>.\nReturns the new TableMetaBuilder after applying given …\nall column names should be added.\ncolumn requests should be added after already exist …\ncolumn requests should be added at first place.\ncolumn requests should be added at last place.\nCreate an empty builder, with all fields set to <code>None</code> or …\nCreate an empty builder, with all fields set to <code>None</code> or …\nComment of the table.\nComment of the table.\nComment of the table.\nEngine type of this table. Usually in small case.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the full table name in the form of …\nId and version of the table.\nId and version of the table.\nId and version of the table.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nName of the table.\nName of the table.\nName of the table.\nDeprecated. See …\nTable options.\nTable options.\nTable options.\nOrder doesn’t matter to this array.\nThe indices of columns in primary key. Note that the index …\nThe indices of columns in primary key. Note that the index …\nThe indices of columns in primary key. Note that the index …\nThe indices of columns in primary key. Note that the index …\nSort the columns in RawTableInfo, logical tables require …\nSplit requests into different groups using column location …\nUnique id of this table.\nThe indices of columns in value. Order doesn’t matter to …\nVersion of the table, bumped when metadata (such as …\n<code>TimeRangePredicateBuilder</code> extracts time range from logical …\nReturns the logical exprs.\nlogical exprs\nExtract time range filter from <code>IN (...)</code> expr.\nExtract time range filter from <code>WHERE</code>/<code>IN (...)</code>/<code>BETWEEN</code> …\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCreates a new <code>Predicate</code> by converting logical exprs to …\nEvaluates the predicate against the <code>stats</code>. Returns a …\nAssert the scalar value is not utf8. Returns <code>None</code> if it’…\nBuilds physical exprs according to provided schema.\nAdd column request\nAlter table request\nChange column datatype request\nCopy table request\nDelete (by primary key) request\nTruncate table request\nExtra options that may not applicable to all table engines.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nValues of each column in this table’s primary key and …\nTime-to-live of table. Expired data will be automatically …\nReturns true if the <code>key</code> is a valid key for any engine or …\nMemtable size of memtable.\nStatistics for a column within a relation\nStatistics for a relation Fields are optional and can be …\nStatistics on a column level\nNumber of distinct values\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nIf true, any field that is <code>Some(..)</code> is the actual value in …\nMaximum value of column\nMinimum value of column\nNumber of null values on column\nThe number of table rows\ntotal bytes of the table rows\nTable handle.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nGet primary key columns in the definition order.\nGet a reference to the schema for this table.\nTests whether the table provider can make use of any or …\nGet a reference to the table info.\nGet the type of this table for metadata/catalog purposes.\nAdapt greptime’s TableRef to DataFusion’s TableProvider…\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nThis metrics struct is used to record and hold memory usage\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCreate a new MemoryUsageMetrics structure, and set …\nRecord the end time of the query\nnumbers table for test\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nAdapt greptime’s SendableRecordBatchStream to GreptimeDB…\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nRepresents a resolved path to a table of the form …\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCreates a 1 column 100 rows table, with table name “…\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.")