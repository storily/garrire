FROM rust:alpine AS calc

RUN apk add bash build-base

COPY calc /build/calc
COPY bin/build-calc /build/bin/build-calc

WORKDIR /build
RUN RUSTFLAGS="-C target-feature=-crt-static" \
	bin/build-calc "/build/calc.h" "/webroot/app/Ext/libcalc.so"


FROM alpine

RUN apk add \
	composer \
	php \
	php-ctype \
	php-fpm \
	php-intl \
	php-tokenizer

WORKDIR /webroot
COPY app /webroot/app
COPY bin /webroot/bin
COPY composer.json composer.lock /webroot/
COPY --from=calc /build/calc.h /build/calc/target/release/libcalc.so /webroot/app/Ext/

ENV PHP_ENV production

RUN composer install
RUN rm -r /etc/php7/* \
	&& addgroup -Sg 1300 www-data \
	&& adduser -SDHG www-data -u 1300 www-data
COPY prod/php.ini prod/php-fpm.conf /etc/php7/

CMD ["php-fpm7"]
