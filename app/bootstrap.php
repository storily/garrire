<?php

declare(strict_types=1);

define('ROOT', realpath(__DIR__ . '/../'));

require_once(ROOT.'/vendor/autoload.php');

use Illuminate\Database\Capsule\Manager as Capsule;
use Illuminate\Events\Dispatcher;
use Illuminate\Container\Container;

function error_dump($arg): void
{
  error_log(var_export($arg, true));
}

$env_sh = "env -i bash -c 'cd ".ROOT." && source env.sh && env'";
foreach (array_filter(
	array_map(
		fn ($line) => array_filter(explode('=', $line, 2)),
		explode("\n", `$env_sh`)
	),
	fn ($pair) => !!$pair && !in_array($pair[0], ['_', 'PWD', 'SHLVL'])
) as $envpair) $_ENV[$envpair[0]] = $envpair[1];

define('ENVIRONMENT', $_ENV['PHP_ENV']);
const DEVELOPMENT = 'development';
const PRODUCTION = 'production';

$capsule = new Capsule;
$capsule->addConnection([
    'driver'    => 'mysql',
    'host'      => $_ENV['DATABASE_HOST'],
    'database'  => $_ENV['DATABASE_NAME'],
    'username'  => $_ENV['DATABASE_USER'],
    'password'  => ($pf = $_ENV['DATABASE_PASSWORD_FILE'] ?? null) ? trim(file_get_contents($pf)) : $_ENV['DATABASE_PASSWORD'],
]);

$capsule->setEventDispatcher(new Dispatcher(new Container));
$capsule->setAsGlobal();
$capsule->bootEloquent();
