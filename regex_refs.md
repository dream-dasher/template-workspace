# Convenience Regex References
- Quick use of more elaborate regi.
- Reminder of useful patterns.

**NOTE**: *newline* may be grabbed when copying.

# tracing

## tracing events & spans
events
```regex
( |^|\t)tea::(trace|debug|info|warn|error|event)!\(
( |^|\t)(trace|debug|info|warn|error|event)!\(
```
spans
```regex
( |^|\t)tea::(trace_|debug_|info_|warn_|error_)?span!\(
( |^|\t)(trace_|debug_|info_|warn_|error_)?span!\(
```
event & spans
```regex
( |^|\t)tea::((trace|debug|info|warn|error)(_span)?|event|span)!\(
( |^|\t)((trace|debug|info|warn|error)(_span)?|event|span)!\(
```

# tests

## attributes
```regex
#\[ignore
```
