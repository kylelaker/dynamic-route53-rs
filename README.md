# Route 53 Dynamic DNS Client

This is a client that interacts with the Route 53 API using the AWS SDK for Rust
to dynamically update a DNS record.

## Getting Started

### Setting up IAM and Notifications

Since this project interacts with the Route 53 API directly, it will be necessary to
have an IAM principal with permissions to invoke `route53:ChangeResourceRecordSets`
on the hosted zone where the dynamic DNS record will be. This is the only required
permission, but unfortunately Route 53 does not provide resource specifiers for
particular Resource Record Sets nor any condition keys which means the created
principal will need access to modify any RR set in the Hosted Zone. Consider this
potential overly-permissive setup in your Route 53 Hosted Zone design.

There is an included sample CloudFormation template,
`user-and-notification.template.yml` which creates the IAM user as well as the
necessary infrastructure to send an email every time that user updates the given
Route 53 Hosted Zone. The notifications are based on EventBridge and SNS with a
Lambda function to transform the event into a more useful message. It will be
necessary to create the Access Key for the user manually.

### Building the project

The project can be build with `cargo build`, optionally with `cargo build --release`.

### Running the project

A configuration file based on `sample_config.yml` needs to be created. This should
provide the specific values defined in the sample. Then, export your AWS credentials
using the standard environment variable names.

You can then run the command with:

```
$ ./target/debug/dynamic-route53 -c <FILE>
```

which should properly update the entry in Route 53.

### On Arch Linux

A (less-than-great) PKGBUILD is provided that will put the systemd files in place
to run this on a timer. Install the package using the PKGBUILD as you usually would.
Update the configuration files in `/etc/dynamic-route53` and then enable the timer with

```
# systemctl enable dynamic-route53.timer
```


## To Do

- [ ] Handle errors more correctly
- [ ] Support IPv6/AAAA
- [ ] Improve systemd configs
- [ ] Consider actually daemonizing rather than using a `systemd.timer(5)`
- [ ] Docs