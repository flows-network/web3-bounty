This is the web frontend for the bounty program. 

1. Ask the user to log into MetaMask wallet and retrieve a list of `accounts`.
2. Query a private airtable table using the [query](../query/) serverless function to look up records for `accounts[0]`.
3. Display a list of available (no records) "Log in with XYZ" buttons. Each XYZ has a flow function in this repo. It
  1. Goes through the OATH process.
  2. Saves user profile and `accounts` into airtable.
  3. Returns instruction to close the table and return to this page (to avoid re-connecting MetaMask at page reload).
5. The user clicks on one button and goes through the process to connect. Close tab and comes back to this page after OATH completes.
6. Repeat from #2 above.
