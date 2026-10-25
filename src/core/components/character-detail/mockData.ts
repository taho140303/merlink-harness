import type { CharacterFullProfile } from './types';

export const rimuruProfile: CharacterFullProfile = {
  id: 'rimuru-tempest',
  basicInfo: {
    name: 'Rimuru Tempest',
    species: 'Ultimate Slime (True Dragon-tier)',
    biologicalSex: 'none',
    genderIdentity: 'NONE',
    genderExpression: 'NONE',
  },
  appearance: {
    height: 170,
    weight: 60,
    hair: { value: 'Silver-blue', hex: '#A8C0D6' },
    eyes: { value: 'Golden', hex: '#F5C518' },
    skin: { value: 'Pale', hex: '#F5E6D3' },
    hairNote:
      'Long silver-blue hair, typically loose or in a ponytail. Shifts subtly with mood.',
    eyesNote:
      'Golden eyes that shimmer. Can shift hue when expressing strong emotion.',
    skinNote:
      'Pale, smooth, androgynous. Reshapable via slime physiology.',
    outfitStyle:
      'Casual-elegant. Blue kimono-style top with black pants, rarely seen in formal attire.',
    defaultSummary:
      'A humanoid slime with silver-blue hair and golden eyes. Androgynous beauty, calm and gentle presence.',
  },
  distinguishingMarks: [
    { id: 'm1', content: 'Predator skill mark subtly visible on upper chest when active' },
    { id: 'm2', content: 'Blue-tinted slime core glow under certain light' },
    { id: 'm3', content: 'Slight magicule aura (visible to magical sight)' },
  ],
  relationships: [
    {
      id: 'r1',
      personName: 'Veldora Tempest',
      role: 'Equal',
      category: 'Social',
      states: {
        trust: 'ABSOLUTE',
        affection: 'HIGH',
        hostility: 'NONE',
        fear: 'NONE',
        respect: 'HIGH',
        familiarity: 'ABSOLUTE',
        dependency: 'MODERATE',
      },
    },
    {
      id: 'r2',
      personName: 'Great Sage',
      role: 'Equal',
      category: 'Social',
      states: {
        trust: 'ABSOLUTE',
        affection: 'MODERATE',
        hostility: 'NONE',
        fear: 'NONE',
        respect: 'HIGH',
        familiarity: 'ABSOLUTE',
        dependency: 'HIGH',
      },
    },
    {
      id: 'r3',
      personName: 'Shion',
      role: 'Inferior',
      category: 'Social',
      states: {
        trust: 'HIGH',
        affection: 'HIGH',
        hostility: 'NONE',
        fear: 'LOW',
        respect: 'HIGH',
        familiarity: 'HIGH',
        dependency: 'LOW',
      },
    },
    {
      id: 'r4',
      personName: 'Benimaru',
      role: 'Inferior',
      category: 'Social',
      states: {
        trust: 'HIGH',
        affection: 'MODERATE',
        hostility: 'NONE',
        fear: 'NONE',
        respect: 'HIGH',
        familiarity: 'HIGH',
        dependency: 'LOW',
      },
    },
    {
      id: 'r5',
      personName: 'Milim Nava',
      role: 'Equal',
      category: 'Social',
      states: {
        trust: 'HIGH',
        affection: 'HIGH',
        hostility: 'NONE',
        fear: 'LOW',
        respect: 'MODERATE',
        familiarity: 'HIGH',
        dependency: 'LOW',
      },
    },
    {
      id: 'r6',
      personName: 'Hinata Sakaguchi',
      role: 'Equal',
      category: 'Rival',
      states: {
        trust: 'MODERATE',
        affection: 'MODERATE',
        hostility: 'LOW',
        fear: 'NONE',
        respect: 'HIGH',
        familiarity: 'MODERATE',
        dependency: 'NONE',
      },
    },
  ],
  knowledge: [
    { id: 'k1', type: 'Initial', path: 'Memories of modern Japan', certainty: 'High' },
    { id: 'k2', type: 'Initial', path: 'Great Sage skill database', certainty: 'High' },
    { id: 'k3', type: 'Lore', path: 'Jura Forest historical records', certainty: 'Medium' },
    { id: 'k4', type: 'Lore', path: 'Demon Lord council intel', certainty: 'Low' },
  ],
  competences: [
    {
      id: 'c1',
      name: 'Predator',
      domain: 'Unique Skill',
      level: 'Ultimate',
      reliability: 'High',
      description: 'Absorbs and analyzes targets, gaining their abilities.',
      experience: 'Used extensively since reincarnation.',
      capabilities: [
        'Analyze and copy skills',
        'Store objects in stomach dimension',
        'Mimicry of absorbed beings',
      ],
      limitations: [
        'Requires analysis time for complex targets',
        'Cannot replicate non-physical traits',
        'Weakness to holy attributes',
      ],
    },
    {
      id: 'c2',
      name: 'Great Sage',
      domain: 'Ultimate Skill',
      level: 'Ultimate',
      reliability: 'High',
      description:
        'An intelligence capable of independent thought and parallel processing.',
      experience: 'Constant companion since reincarnation.',
      capabilities: [
        'Parallel calculation',
        'Instant skill analysis',
        'Multi-tasking across battles',
      ],
      limitations: [
        "Limited by Rimuru's own comprehension",
        'Cannot act on decisions Rimuru rejects',
        'Occasional moral detachment',
      ],
    },
    {
      id: 'c3',
      name: 'Universal Shapeshift',
      domain: 'Ultimate Skill',
      level: 'Ultimate',
      reliability: 'High',
      description: 'Free-form transformation of body and form.',
      experience: 'Mastered over time.',
      capabilities: [
        'Mimic any absorbed being',
        'Adjust physical parameters',
        'Slime body flexibility',
      ],
      limitations: [
        'Requires absorbed template',
        'Energy cost for sustained shifts',
      ],
    },
    {
      id: 'c4',
      name: 'Gluttony',
      domain: 'Ultimate Skill',
      level: 'Ultimate',
      reliability: 'High',
      description: 'Mass consumption and energy conversion.',
      experience: 'Awakened during the Orc Lord arc.',
      capabilities: ['Mass absorption', 'Energy redistribution to allies'],
      limitations: ['Risk of overconsumption', 'Cannot process holy energy'],
    },
  ],
  memories: [
    {
      id: 'mem1',
      summary: 'Death in a hospital and reincarnation as a slime',
      type: 'Episodic',
      status: 'Active',
      recollection:
        'Dying alone in a hospital bed after being stabbed protecting a colleague. Voices fading. Then, a strange darkness and a new form - a slime.',
      interpretation:
        'The moment everything changed. The threshold between who I was and who I am.',
      certainty: 'High',
      knowledgeSource: 'Personal experience',
      occurredAt: 'Before reincarnation / Day 1',
      perceivedAt: 'Day 1',
      references: [
        { role: 'Unknown', name: 'The colleague' },
        { role: 'Unknown', name: 'The stranger with the knife' },
      ],
      emotions: [
        { name: 'Loneliness', intensity: 90 },
        { name: 'Fear', intensity: 70 },
        { name: 'Relief', intensity: 40 },
      ],
      connections: [],
      revisions: [
        {
          revisionNumber: 1,
          reason: 'Initial memory',
          change: 'Recorded as-is at reincarnation.',
        },
      ],
    },
    {
      id: 'mem2',
      summary: 'Meeting Veldora in the sealed cave',
      type: 'Episodic',
      status: 'Active',
      recollection:
        'Veldora, the Storm Dragon, sealed in a cave. We talked about the world, about storms, about manga. I gave him a name and he gave me his.',
      interpretation: 'My first true bond. The start of everything I built.',
      certainty: 'High',
      knowledgeSource: 'Personal experience',
      occurredAt: 'Day 2',
      perceivedAt: 'Day 2',
      references: [{ role: 'Friend', name: 'Veldora Tempest' }],
      emotions: [
        { name: 'Joy', intensity: 95 },
        { name: 'Curiosity', intensity: 80 },
        { name: 'Excitement', intensity: 85 },
      ],
      connections: [
        {
          label: 'Reinforced by',
          targetMemoryId: 'mem1',
          targetSummary: 'Death in a hospital',
        },
      ],
      revisions: [
        { revisionNumber: 1, reason: 'Initial memory', change: 'Recorded as-is.' },
      ],
    },
    {
      id: 'mem3',
      summary: 'The Orc Lord and the first real war',
      type: 'Episodic',
      status: 'Active',
      recollection:
        "The Orc Lord, Gelmud's schemes, the fall of the Lizardmen and Ogres. The weight of leading allies into war. The decision to take lives.",
      interpretation:
        "The moment I learned that kindness alone wasn't enough to protect people.",
      certainty: 'High',
      knowledgeSource: 'Personal experience',
      occurredAt: 'Jura Forest War',
      perceivedAt: 'Jura Forest War',
      references: [
        { role: 'Ally', name: 'Benimaru' },
        { role: 'Ally', name: 'Shion' },
        { role: 'Enemy', name: 'Orc Lord Gelmud' },
      ],
      emotions: [
        { name: 'Determination', intensity: 90 },
        { name: 'Grief', intensity: 75 },
        { name: 'Guilt', intensity: 60 },
      ],
      connections: [],
      revisions: [
        { revisionNumber: 1, reason: 'Initial memory', change: 'Recorded as-is.' },
        {
          revisionNumber: 2,
          reason: 'Post-war reflection',
          change: 'Reframed guilt into responsibility.',
        },
      ],
    },
    {
      id: 'mem4',
      summary: 'The fall of Shion and the awakening of the Demon Lord Seed',
      type: 'Episodic',
      status: 'Active',
      recollection:
        'Shion, dead in my arms. The rage. The choice to become a Demon Lord to bring her back. The coldness of the decision.',
      interpretation: 'I would do it again. That terrifies me.',
      certainty: 'High',
      knowledgeSource: 'Personal experience',
      occurredAt: 'Falmuth invasion aftermath',
      perceivedAt: 'Falmuth invasion aftermath',
      references: [
        { role: 'Friend', name: 'Shion' },
        { role: 'Enemy', name: 'Falmuth Kingdom' },
      ],
      emotions: [
        { name: 'Rage', intensity: 100 },
        { name: 'Despair', intensity: 95 },
        { name: 'Resolve', intensity: 100 },
      ],
      connections: [],
      revisions: [
        { revisionNumber: 1, reason: 'Initial memory', change: 'Recorded as-is.' },
      ],
    },
  ],
  psychological: {
    coreTrauma: {
      theWound:
        'Died alone in a hospital after a life of isolation. No one remembered his name. No one came to his funeral. His greatest achievements were hollow. Then, waking as a slime - alone again, in a world with no familiar faces.',
      theLie:
        'If I give enough to others, I will never be alone again. If I am strong enough, nothing I love will ever be taken from me.',
      fundamentalFear:
        'Ending up alone again - loved by no one, remembered by no one. And under that: that the people he loves only love what he gives them, not who he is.',
    },
    cognitiveMoral: {
      moralAlignment: 'Neutral Good',
      moralCode:
        'Protect lives above all. Give everyone a chance. Build a world where no one has to be alone. Never sacrifice those who trust me - even if it costs me my own soul.',
      cognitiveBias:
        'Over-attribution of responsibility - believes every death in his sphere is his failure. Tends to assume the best of allies and the worst of himself.',
      breakingPoint:
        'Deliberate harm to those he has named. He becomes colder, more methodical, and far more dangerous than any demon lord - because he stops negotiating with his own conscience.',
    },
    narrativeEngine: {
      externalWant:
        'To build a nation where all races can live in peace - Tempest, a home that no one has to leave.',
      internalNeed:
        'To believe he is worthy of love without having to earn it, and to allow himself to be protected rather than always being the protector.',
      internalConflict:
        'The pacifist who became a Demon Lord. Every life he takes to protect his people cracks something in him. Every choice to spare an enemy risks the people he loves.',
    },
    behavioralAdaptation: {
      socialMask: {
        personaType:
          'The easygoing, cheerful leader - approachable, humble, playful. Hides the weight of every decision behind a smile and a joke.',
        verbalStyle:
          'Casual and warm. Uses humor to defuse tension. Becomes quiet and precise when serious. Says "hmm" and "well..." when buying time to think.',
      },
      defenseMechanisms: {
        primaryReaction:
          'Diplomatic negotiation first, overwhelming force second. Retreats inward to consult Great Sage when emotionally overwhelmed.',
        stressTrigger:
          'Harm to subordinates. Betrayal by trusted allies. Anyone threatening his nation.',
        copingStrategy:
          'Retreating inward, delegating to Great Sage, then decisive action. Long periods of thought before major decisions. Dark humor with Veldora.',
      },
    },
  },
};
