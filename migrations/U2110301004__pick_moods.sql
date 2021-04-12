CREATE TABLE `pick_moods` (
	`id` INT UNSIGNED NOT NULL AUTO_INCREMENT,
	`created` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
	`text` VARCHAR(255) NOT NULL,
	PRIMARY KEY (`id`),
	UNIQUE KEY `text uniq` (`text`)
);

INSERT INTO `pick_moods` (`text`) VALUES
	('a bit miffed'),
	('abhiman'),
	('abjection'),
	('acceptance'),
	('acedia'),
	('admiration'),
	('adoration'),
	('affection'),
	('aggravation'),
	('aggressiveness'),
	('agitation'),
	('agony'),
	('alarm'),
	('alertness'),
	('alienation'),
	('amae'),
	('amazement'),
	('ambiguphobia'),
	('ambivalence'),
	('amusement'),
	('anger'),
	('angst'),
	('anguish'),
	('annoyance'),
	('anticipation'),
	('anxiety'),
	('apalled'),
	('apathetic'),
	('apathy'),
	('apprehension'),
	('arousal'),
	('arrogance'),
	('assertiveness'),
	('astonishment'),
	('attentiveness'),
	('attraction'),
	('audacity'),
	('avarice'),
	('aversion'),
	('awe'),
	('awumbuk'),
	('bafflement'),
	('basorexia'),
	('beautiful'),
	('befuddlement'),
	('bewilderment'),
	('bitterness'),
	('bittersweetness'),
	('bliss'),
	('boredom'),
	('brabant'),
	('brazenness'),
	('broodiness'),
	('calm'),
	('calmness'),
	('carefree'),
	('carelessness'),
	('caring'),
	('caution'),
	('charity'),
	('cheekiness'),
	('cheerfulness'),
	('cheesed off'),
	('claustrophobia'),
	('coercion'),
	('collywobbles'),
	('comfort'),
	('compassion'),
	('compersion'),
	('competence'),
	('confidence'),
	('confusion'),
	('contempt'),
	('contentment'),
	('courage'),
	('cowardness'),
	('crosspatch'),
	('cruelty'),
	('curiosity'),
	('cyberchondria'),
	('cynicism'),
	('dead inside'),
	('defeatism'),
	('dejection'),
	('delight'),
	('dépaysement'),
	('depression'),
	('derisiveness'),
	('desire'),
	('despair'),
	('detachment'),
	('determination'),
	('disappointment'),
	('disapproval'),
	('discomfort'),
	('discontentment'),
	('disgruntlement'),
	('disgust'),
	('dislike'),
	('dismay'),
	('dispirit'),
	('displeasure'),
	('dissatisfaction'),
	('distraction'),
	('distress'),
	('distrust'),
	('dolce far niente'),
	('dominance'),
	('doom'),
	('doubt'),
	('dread'),
	('dumbstruckness'),
	('eagerness'),
	('ecstasy'),
	('elation'),
	('embarrassment'),
	('empathy'),
	('enjoyment'),
	('enlightenment'),
	('ennui'),
	('enthrallment'),
	('enthusiasm'),
	('envy'),
	('epiphany'),
	('euphoria'),
	('exasperation'),
	('excitement'),
	('exhilaration'),
	('expectancy'),
	('faith'),
	('fatalism'),
	('fear'),
	('feeling gay'),
	('feeling good about oneself'),
	('feeling like a fraud'),
	('ferocity'),
	('fey'),
	('focus'),
	('fondness'),
	('formality'),
	('friendliness'),
	('fright'),
	('frisson'),
	('frozenness'),
	('frustration'),
	('fury'),
	('gaiety'),
	('gentleness'),
	('gezelligheid'),
	('gladness'),
	('gladsomeness'),
	('glee'),
	('gloat'),
	('gloominess'),
	('glumness'),
	('going postal'),
	('gratification'),
	('gratiousness'),
	('gratitude'),
	('greed'),
	('greng jai'),
	('grief'),
	('grouchy'),
	('grumpy'),
	('guilt'),
	('habituation'),
	('han'),
	('happiness'),
	('hate'),
	('hatred'),
	('heebie-jeebies'),
	('helplessness'),
	('hiraeth'),
	('homefulness'),
	('homesickness'),
	('hopefulness'),
	('hopelessness'),
	('horror'),
	('hospitality'),
	('hostility'),
	('humbleness'),
	('humiliation'),
	('humility'),
	('humour'),
	('hunger'),
	('hurt'),
	('hwyl'),
	('hysteria'),
	('idleness'),
	('ijirashi'),
	('ilinx'),
	('impatience'),
	('impiety'),
	('in a huff'),
	('indifference'),
	('indignation'),
	('infatuation'),
	('inhabitiveness'),
	('inhospitality'),
	('insecurity'),
	('insight'),
	('insult'),
	('interest'),
	('intrigue'),
	('irritability'),
	('irritation'),
	('isolation'),
	('jealousy'),
	('jolliness'),
	('joviality'),
	('joy'),
	('jubilation'),
	('kaukokaipuu'),
	('kindness'),
	('kitsch'),
	('l’appel du vide'),
	('laxity'),
	('liget'),
	('liking'),
	('litost'),
	('loathing'),
	('loneliness'),
	('longing'),
	('looseness'),
	('love'),
	('lust'),
	('malu'),
	('man'),
	('matutolypea'),
	('meekness'),
	('mehameha'),
	('melancholy'),
	('miserliness'),
	('misery'),
	('modesty'),
	('mono no aware'),
	('morbid curiosity'),
	('morbidness'),
	('mortification'),
	('mulishness'),
	('nakhes'),
	('nastiness'),
	('neglect'),
	('nervousness'),
	('nginyiwarrarringu'),
	('nostalgia'),
	('obstinacy'),
	('oime'),
	('optimism'),
	('outrage'),
	('overwhelm'),
	('panic'),
	('paranoia'),
	('passion'),
	('patience'),
	('pensiveness'),
	('perseverance'),
	('perversity'),
	('pessimism'),
	('peur des espaces'),
	('philoprogenitiveness'),
	('pique'),
	('pity'),
	('pleasure'),
	('politeness'),
	('possessiveness'),
	('powerlessness'),
	('pride'),
	('pronoia'),
	('prudishness'),
	('puzzlement'),
	('rage'),
	('rapture'),
	('rashness'),
	('recklessness'),
	('regret'),
	('rejection'),
	('relaxation'),
	('relief'),
	('reluctance'),
	('remorse'),
	('reproachfulness'),
	('resentment'),
	('resignation'),
	('restlessness'),
	('reverence'),
	('revulsion'),
	('rightfulness'),
	('ringxiety'),
	('rivalry'),
	('road rage'),
	('ruinenlust'),
	('ruthlessness'),
	('sadness'),
	('satisfaction'),
	('saudade'),
	('schadenfreude'),
	('scorn'),
	('self-appraisal'),
	('self-consciousness'),
	('self-deprecative'),
	('self-pity'),
	('sentimentality'),
	('serenity'),
	('sexual desire'),
	('shame'),
	('shamelessness'),
	('shock'),
	('shyness'),
	('silliness'),
	('smugness'),
	('social connection'),
	('song'),
	('sorrow'),
	('spite'),
	('stress'),
	('stubbornness'),
	('sublime'),
	('submission'),
	('suffering'),
	('sullenness'),
	('surprise'),
	('suspense'),
	('suspicion'),
	('sweet idleness'),
	('sympathy'),
	('technostress'),
	('temerity'),
	('tenderness'),
	('tension'),
	('terror'),
	('thankfulness'),
	('the desire to disappear'),
	('the urge to hoard'),
	('thrill'),
	('thunderstruckness'),
	('tolerance'),
	('torment'),
	('torschlusspanik'),
	('toska'),
	('triumph'),
	('troubledness'),
	('trust'),
	('unbelief'),
	('uncertainty'),
	('uneasiness'),
	('unhappiness'),
	('vengeance'),
	('vengefulness'),
	('vergüenza ajena'),
	('viciousness'),
	('victory'),
	('vigilance'),
	('viraha'),
	('vulnerability'),
	('wanderlust'),
	('warm glow'),
	('woe'),
	('wonder'),
	('wonderstruckness'),
	('worry'),
	('wrath'),
	('yucky'),
	('żal'),
	('zeal'),
	('zest')
;
