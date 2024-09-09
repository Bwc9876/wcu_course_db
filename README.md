# WCU Course DB

```mermaid
graph LR;
A[Web request to catalog.wcupa.edu]-->B[Regex horrors that made me cry]
B-->C[JSON]
C-->D[Construct graph of pre reqs and output to .dot file]
D-->E[Convert .dot to .pdf using dot cli]
```
