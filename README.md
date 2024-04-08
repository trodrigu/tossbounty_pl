# Tossbounty Program Library
This is a library for the Tossbounty program. It is a program that allows you to create a bounty for a task and reward the person who completes it. Upon bounty claim the program can fire off a pause to the integrated protocol in order to prevent further hacks.  It makes use of a hard-coded registry of programs that are allowed to create bounties.

# Pausing
Since arbitrary CPI is considered a security risk, we create a new program per organization which uses an Anchor attribute to validate the program ID.
Although this is less convenient since it requires code changes and a new deployment it is more secure.
