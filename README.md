# ut

Ever sat there with a unix timestamp and had to use some sketchy website to turn it into something human readable? Ever had a unix timestamp in ms, having to chop off the last 3 digits to get said sketchy website to return a date prior to the heat death of the universe?

No more!

Use `ut <timestamp>` and get something you can understand, regardless of your timestamp being seconds or ms.

```
$ git clone git@github.com:aclemmensen/ut.git
$ cargo install --path .
$ ut 1695767485266
2023-09-26 22:31:25 UTC
```