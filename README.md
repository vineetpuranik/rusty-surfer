# rusty-surfer
Initial set of requirements 
1. Write a Rust program that takes a URL as input from the user via the command line.
2. Use a headless browser library in Rust, such as headless_chrome or rust-headless-chromium, to load the web page at the given URL and monitor its network requests.
3. As the web page is loaded and its resources are requested, store the remote IP addresses in a set or a hash map to eliminate duplicates.
4. For each remote IP address, use a whois lookup API to obtain information about the organization that owns the IP address range, the location of the IP address, and any other relevant information.
5. Print out the list of unique remote IP addresses along with the information obtained from the whois lookup API.
