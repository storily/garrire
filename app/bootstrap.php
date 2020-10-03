<?php

declare(strict_types=1);

define('ROOT', realpath(__DIR__ . '/../'));

require_once(ROOT.'/vendor/autoload.php');

use Doctrine\ORM\Tools\Setup;
use Doctrine\ORM\EntityManager;

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

define('DEV', $_ENV['PHP_ENV'] == 'development');

$entity_manager = EntityManager::create([
	'url' => ($pf = $_ENV['DATABASE_URL_FILE'] ?? null) ? trim(file_get_contents($pf)) : $_ENV['DATABASE_URL'],
], Setup::createAnnotationMetadataConfiguration([ROOT.'/app/entities'], DEV));
