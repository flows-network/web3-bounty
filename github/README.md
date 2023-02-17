The GitHub login flow function. It is deployed as a public webhook. It's public URL is used as the callback URL for GitHub OATH. This function:

* Receives callback from GitHub OATH with an `access` token and a custom param that contains a list of `accounts`.
* Figures out user profile and creates an airtable record for each address in the `accounts`.

Returns instruction to close this tab and go to the frontend UI page.

