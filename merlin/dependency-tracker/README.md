# Dependency Tracker

1. Create parser candidate scheduler

   This stage is essential to solve this special case
   
       Special case: re-export all `export * from 'some/path'`
       Can't parse unless 'some/path' is already parsed.

2. Create dependency graph

Phase1: Extract all symbols along with their dependency

Phase2: Add reversed pointers (used_by) to each symbol

Phase3: Save the graph to file (so we don't need to create the graph again)

3. Get symbols' dependencies

Traverse the dependency graph by following the used_by pointers
