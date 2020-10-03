<?php

declare(strict_types=1);
require_once('./app/bootstrap.php');

use Doctrine\ORM\Tools\Console\ConsoleRunner;

return ConsoleRunner::createHelperSet($entity_manager);
