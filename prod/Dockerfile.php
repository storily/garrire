FROM rust:alpine AS calc

RUN apk add bash build-base

COPY calc /build/calc
COPY bin/build-calc /build/bin/build-calc

WORKDIR /build
RUN RUSTFLAGS="-C target-feature=-crt-static" \
	bin/build-calc "/build/calc.h" "/webroot/app/Ext/libcalc.so"


FROM php:7.4-fpm-alpine

COPY --from=composer /usr/bin/composer /usr/bin/composer
RUN apk add icu icu-dev libcurl curl-dev

RUN mv "$PHP_INI_DIR/php.ini-production" "$PHP_INI_DIR/php.ini"
COPY prod/php.ini "$PHP_INI_DIR/conf.d/zzz-sassbot.ini"
COPY prod/php-fpm.conf /usr/local/etc/php-fpm.d/sassbot.conf

RUN docker-php-ext-install \
	ctype \
	curl \
	iconv \
	intl \
	pdo_mysql \
	pdo_sqlite

RUN docker-php-ext-enable \
	opcache \
	sodium \
	tidy \
	xsl

WORKDIR /webroot
ENV PHP_ENV production
ENV APP_DEBUG false

COPY app /webroot/app
COPY bin /webroot/bin
COPY composer.json composer.lock /webroot/
COPY --from=calc /build/calc.h /build/calc/target/release/libcalc.so /webroot/app/Ext/

RUN composer install
