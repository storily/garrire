<?php

declare(strict_types=1);
namespace Models;

use GuzzleHttp\Client;

class NanowrimoSetting extends Model
{
	private static $client = null;

	protected $fillable = ['name', 'value'];

	public static function get_client(): Client
	{
		$expiry = static::where('name', 'auth-expiry')->first()->value ?? null;
		if ($expiry) $expiry = new \DateTime("@$expiry");
		if ($expiry && $expiry > new \DateTime) return static::$client ?? static::make_client();

		// get a new one
		$login = static::whereIn('name', ['login-user', 'login-password'])
			->get()
			->flatMap(fn ($row) => [$row->name => $row->value])
			->toArray();
		$user = $login['login-user'] ?? null;
		$pass = $login['login-password'] ?? null;
		if (!$user || !$pass) throw new \Exception('no login info!');
		$res = (new Client)->post('https://api.nanowrimo.org/users/sign_in', [
			'json' => [
				'identifier' => $user,
				'password' => $pass,
			],
		]);
		if (($code = $res->getStatusCode()) > 399) throw new \Exception("login failed with status $code");

		$data = json_decode($res->getBody()->getContents());
		$token = $data->auth_token ?? null;
		if (!$token) throw new \Exception('no token returned from login call');

		$exp = json_decode(base64_decode(explode('.', $token)[1] ?? '') ?: 'null', true)['exp'] ?? null;
		if (!$exp) throw new \Exception('no exp field in token');

		static::updateOrCreate(['name' => 'auth-token'], ['value' => $token]);
		static::updateOrCreate(['name' => 'auth-expiry'], ['value' => "$exp"]);

		return static::make_client();
	}

	private static function make_client(): Client
	{
		$token = static::where('name', 'auth-token')->first()->value ?? null;
		if (!$token) throw new \Exception('no auth token');

		return static::$client = new Client([
			'base_uri' => 'https://api.nanowrimo.org',
			'headers' => [
				'Authorization' => $token,
			],
		]);
	}
}
