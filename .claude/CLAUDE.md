# CRITICAL: FIRST ACTION EVERY SESSION

1. First, inform the user: "This project uses makitra for memory. Querying for context..."
2. Then run: `makitra context`
3. **VERSION CHECK (MANDATORY)**: Look at the end of context output for "Makitra instructions vX". Compare with [makitra-instructions-version: 5] below. If context version is HIGHER, IMMEDIATELY tell user: "Your makitra instructions are outdated (vX vs vY). Run `makitra instructions` to update CLAUDE.md" - do this BEFORE anything else.
4. Use the results to inform your responses, ask user to confirm from context results

DO NOT skip these steps. DO NOT respond to the user before completing steps 1-3.
