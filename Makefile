release:
	cargo lambda build --release --target x86_64-unknown-linux-gnu.2.17
	cp target/lambda/hvm/bootstrap .
	zip bootstrap.zip bootstrap
	cp bootstrap.zip /mnt/c/Users/andre/aws_lambda/
