/**
 * Sample workspace data, ported from the mock export in ref-ui/shared.js.
 *
 * Provider data is intentionally absent: providers now come from the Rust
 * backend (`load_providers`) or the browser-safe mock in `src/lib/tauri.ts`.
 */
import type { Character, Chat, Chapter, Project } from "../types";

interface ProjectSeed {
  id: string;
  title: string;
  genre: string;
  motifs: string[];
  cover: string;
  status: string;
  updatedAt: string;
  wordCount: number;
  description: string;
  chapters: string[];
}

const projectSeeds: ProjectSeed[] = [
  {
    id: "glass-tide",
    title: "The Glass Tide",
    genre: "Literary fantasy",
    motifs: ["Memory", "Tides", "Belonging"],
    cover: "linear-gradient(145deg,#29495b 0%,#7696a8 48%,#d9b8aa 100%)",
    status: "In progress",
    updatedAt: "2 hours ago",
    wordCount: 42680,
    description:
      "On a coast where the sea returns forgotten memories, a mapmaker searches for the truth her family buried.",
    chapters: [
      "The Shore of Lost Things",
      "A Map Drawn in Salt",
      "What the Water Kept",
      "The Lighthouse Keeper",
    ],
  },
  {
    id: "midnight-orchard",
    title: "The Midnight Orchard",
    genre: "Magical realism",
    motifs: ["Family", "Seasons", "Secrets"],
    cover: "linear-gradient(145deg,#252d4e 0%,#826588 52%,#e8b58b 100%)",
    status: "In progress",
    updatedAt: "Yesterday",
    wordCount: 28140,
    description:
      "Every winter, a forgotten orchard blooms at midnight; this year, its fruit bears the names of the missing.",
    chapters: [
      "A Harvest Out of Season",
      "Names Beneath the Bark",
      "The House on Marigold Lane",
    ],
  },
  {
    id: "atlas-of-small-worlds",
    title: "Atlas of Small Worlds",
    genre: "Science fiction",
    motifs: ["Discovery", "Home", "Wonder"],
    cover: "linear-gradient(145deg,#253c5e 0%,#64899f 53%,#d1d6bd 100%)",
    status: "Draft",
    updatedAt: "3 days ago",
    wordCount: 18420,
    description:
      "A cartographer of impossible spaces discovers a city folded between two pages of her father’s atlas.",
    chapters: [
      "The Door in the Margin",
      "Coordinates for Elsewhere",
      "A City the Size of a Promise",
    ],
  },
  {
    id: "after-the-lanterns",
    title: "After the Lanterns",
    genre: "Contemporary",
    motifs: ["Second chances", "Music", "City life"],
    cover: "linear-gradient(145deg,#665269 0%,#bd8581 48%,#edc8a6 100%)",
    status: "Revising",
    updatedAt: "5 days ago",
    wordCount: 56320,
    description:
      "Two estranged musicians meet beneath the last lantern festival in their hometown, a decade after their final song.",
    chapters: [
      "The Last Train Home",
      "An Unfinished Melody",
      "The Space Between Notes",
    ],
  },
  {
    id: "paper-constellations",
    title: "Paper Constellations",
    genre: "Young adult",
    motifs: ["Friendship", "Stars", "Courage"],
    cover: "linear-gradient(145deg,#283561 0%,#747caf 52%,#c3a7a5 100%)",
    status: "Draft",
    updatedAt: "1 week ago",
    wordCount: 12950,
    description:
      "A shy teenager makes constellations from the letters her friends never sent, and finds a map to a new beginning.",
    chapters: [
      "Stars in an Envelope",
      "The Observatory Club",
      "Messages Without Addresses",
    ],
  },
  {
    id: "the-quiet-archive",
    title: "The Quiet Archive",
    genre: "Mystery",
    motifs: ["Truth", "Archives", "Inheritance"],
    cover: "linear-gradient(145deg,#39443f 0%,#85938a 50%,#d2bba2 100%)",
    status: "Complete",
    updatedAt: "2 weeks ago",
    wordCount: 72410,
    description:
      "An archivist inherits a locked library containing every letter her town tried to forget.",
    chapters: [
      "The Unmarked Key",
      "Catalog of Silences",
      "A Letter in Blue Ink",
    ],
  },
];

const beats: string[][] = [
  [
    "The morning began with a small impossibility that no one else seemed to notice.",
    "By noon, the familiar streets had acquired the uncertain edges of a place remembered incorrectly.",
    "It was not the discovery itself that frightened her; it was how long the evidence had been waiting in plain sight.",
    "Before evening, she understood that the next decision would change the story she had been telling herself.",
  ],
  [
    "She returned to the place she had sworn she would never visit again, carrying only a notebook and an unfinished question.",
    "A stranger at the door knew her name, but not the version of it she had used for years.",
    "Their conversation moved carefully around the truth, like two people making room for a sleeping child.",
    "When the light shifted, she saw the mark in the wall and realized the past had left more than a memory.",
  ],
  [
    "For a moment the world held still, as if waiting to see which story they would choose to believe.",
    "He laid the evidence between them; neither of them reached for it at first.",
    "Outside, ordinary life continued with a confidence that felt almost cruel.",
    "The answer did not make either of them brave, but it gave them somewhere to begin.",
  ],
  [
    "There were three ways to cross the distance, and each demanded a different kind of honesty.",
    "She chose the slow road because it left room to listen for the voices she had mistaken for silence.",
    "At the edge of the city, the first sign appeared exactly where the old notes said it would not.",
    "She folded the map, lifted her eyes, and followed the path that had never been drawn.",
  ],
];

const projectDetails: string[][] = [
  ["Mara", "the returning tide", "her mother’s map", "the lighthouse bell"],
  [
    "Iris",
    "the midnight orchard",
    "her grandmother’s ledger",
    "a pear with a name carved into its skin",
  ],
  [
    "Leonie",
    "the folded city",
    "her father’s atlas",
    "the paper door",
  ],
  [
    "Sofia",
    "the lantern-lit station",
    "the recording they never released",
    "a familiar melody",
  ],
  ["Nell", "the rooftop observatory", "her best friend’s letter", "the penciled stars"],
  ["Ada", "the silent archive", "the sealed correspondence", "the blue-ink note"],
];

const createdAtDates = [
  "2026-08-12",
  "2026-08-29",
  "2026-09-05",
  "2026-06-18",
  "2026-09-21",
  "2026-05-10",
];

const chapterWordCounts = [3210, 4110, 2950, 3820];

/** Complete opening chapters for the flagship project: written prose, not placeholder text. */
const glassTideChapters: string[][] = [
  [
    "The ferry entered Bellwether Harbor beneath a low ceiling of cloud. Mara stood at the rail with one hand on her suitcase and watched the shoreline assemble itself from familiar pieces: the net shed with its green roof, the crooked ladder on the quay, the lighthouse holding its pale head above the rain. Ten years away ought to have made the place smaller. Instead, she felt as though she had shrunk to the size of the girl who had once counted waves here for her mother. When the bell rang across the water, a boy on the deck looked up. There was no wind to carry its sound.",
    "Her mother’s notebook lay at the top of the case because Mara had packed it last. The cover had softened at the corners, and the penciled coastlines inside no longer matched the shore in front of her. That had always been true of Bellwether: every storm changed the shape of a beach, and every spring someone moved a boundary stone. Still, one little mark bothered her. On the newest map, her mother had drawn a path to a cove that appeared nowhere else in the family papers. Beside it were three words pressed hard enough to dent the page: wait for the tide.",
    "At the end of the gangway she stopped to retie a lace that did not need tying. The sand beneath the boards was strewn with sea glass, weed, and the ordinary wreckage of a high tide. Among it lay a blue button with a nick on one edge. Mara picked it up before she had quite decided to move. The button had come from a coat she had worn at thirteen, the winter her father went missing. She remembered losing it on the morning they told her he would not come back. For a long moment she could hear nothing except water moving through the stones.",
  ],
  [
    "Mara spread the maps over the kitchen table, weighing each corner with a cup. Her mother had dated every sheet in a tidy hand, yet the sequence made less sense the longer she studied it. A sandbar disappeared one October and returned the next May, a street bent west when every survey placed it east, and the little cove acquired a new name in blue ink. Saltmere. She ran a finger along its edge and felt a slight ridge beneath the paper. Her mother had folded the page there so often that it remembered the crease, even after years in a drawer.",
    "Elias arrived with a tin of tea and the patient expression of someone who had been called without being invited. He set the tin down when he saw the blue button beside the maps. Neither of them spoke of it. Instead, he asked whether Mara had slept, whether the old stove still smoked in the mornings, whether she had found the spare key. His questions were kind, and their careful order made her wary. When she pointed to Saltmere, he glanced toward the window rather than at the paper. The sea was hidden by houses, but Mara had the feeling he could see it anyway.",
    "They walked out at dusk when the tide began to draw away from the pier. Elias carried a lantern he did not light. The path on the map crossed a patch of empty shore, then turned between two black rocks whose shapes Mara knew by heart. At low water, another path appeared: a seam of pale shells leading to the cove. She had played here as a child and never seen it. Elias said the shoreline changed. Mara thought of her mother's heavy pencil strokes, the button waiting in the sand, and all the years people had told her that remembering was not the same as knowing.",
  ],
  [
    "Saltmere was not large enough to earn a name on most maps. Its stones were smooth and dark, its water colder than the harbor even in summer. Mara knelt beside a shallow pool where the retreating tide had left a child's red ribbon. The color was bright despite the salt, and a knot near its end matched the one her mother had taught her to tie. She had seen that ribbon in a photograph from the year before her father vanished. She looked at Elias, who was standing farther up the beach. He watched her hands but did not ask what she had found.",
    "A sound came across the water then: not a voice, exactly, but the half-remembered rhythm of someone calling her in for supper. Mara shut her eyes. For an instant she was twelve again, barefoot on the kitchen floor, trying to hide sand inside her pockets. The memory arrived with its own weather, its own smell of onions browning in a pan. Then the sensation passed and she was kneeling among wet stones, holding a ribbon she could not explain. Elias crouched beside her and said that sometimes the sea returned what people had lost. He did not say whether he meant objects or something else.",
    "She asked him how long he had known. Elias took time to answer, turning a pebble over in his palm until its dry side was dark. His grandfather had kept a ledger of everything washed up at Saltmere, he said. Most entries were ordinary: a key, a boot, a page torn from a cookbook. Beside a few, however, he had written names and dates that belonged to no shipwreck. Mara's father appeared in that ledger twice. The first entry was from before he went missing. The second, Elias said, had been written only last winter. The tide advanced a little and touched the toe of Mara's shoe.",
  ],
  [
    "The lighthouse smelled of oil, stone dust, and tea steeped too long. Elias led Mara past the narrow sleeping room to a cabinet built into the curved wall. Its lock opened with a key on a length of blue cord. Inside were rows of thin books, their spines labeled by year. She found the winter of her father's disappearance and ran her thumb over the uneven paper before turning a page. Most entries were written in Elias's grandfather's square script. At the bottom of one sheet, a different hand had added a note: the bell must be rung before the water reaches the steps.",
    "The final ledger was scarcely half full. Mara found her father's name beneath a date from eight months ago and read it three times. The entry described a brass compass, its needle fixed toward the open sea, and a folded scrap of music in the case. No one had claimed either item. Elias brought the compass from a drawer and placed it between them without a word. Mara recognized a small gouge near its hinge, the mark she had made by dropping it from the kitchen table as a child. She had believed it was buried with him. The needle did not move when she lifted the case.",
    "From the gallery she could see the harbor lights beginning to appear, each one answering a house on the shore. Elias told her the bell was a warning, not a welcome. When certain things came back with the tide, people followed them into the water, hoping to reach whatever memory had been returned. His grandfather had lost a brother that way. Mara closed the compass and looked at the narrow strip of sand below the tower. She wanted to ask whether her father had followed something, or whether the sea had offered him a way home. Below them, the first wave reached the steps.",
  ],
];

export const mockProjects: Project[] = projectSeeds.map((seed, projectIndex) => {
  const [person, place, object, signal] = projectDetails[projectIndex];
  const chapters: Chapter[] = seed.chapters.map((title, chapterIndex) => ({
    id: `${seed.id}-chapter-${chapterIndex + 1}`,
    title,
    wordCount: chapterWordCounts[chapterIndex],
    status: chapterIndex === seed.chapters.length - 1 ? "Draft" : "Written",
    content: beats[chapterIndex].map((paragraph, paragraphIndex) => {
      const additions = [
        `For ${person}, ${place} had always been a landmark on the edge of childhood; now it seemed to be asking a question. ${paragraph} The air carried a trace of rain, and somewhere beyond the buildings ${signal} gave the hour a shape she could hold.`,
        `She unwrapped ${object} with the care of someone handling a fragile answer. ${paragraph} Every line suggested a different route through what she had been told, and every omission felt more deliberate than the last.`,
        `The thought brought ${person} back to ${place}, where ordinary details had begun to feel like clues. ${paragraph} She wrote the moment down before she could persuade herself it had meant nothing.`,
        `That night ${signal} returned to her thoughts. ${paragraph} She put ${object} beside the window, not because it solved anything, but because she wanted to remember where the search had truly begun.`,
      ];
      return (
        additions[paragraphIndex] +
        " She noticed the small material details that gave the moment its weight: the color of the light on the table, the shape of a shadow beside the door, the pause before someone chose a word. None of them proved anything alone, but together they made it impossible to pretend this day was ordinary."
      );
    }),
  }));

  return {
    ...seed,
    author: "You",
    createdAt: createdAtDates[projectIndex],
    chapters,
  };
});

// The flagship project carries individually written opening chapters.
mockProjects[0].chapters.forEach((chapter, index) => {
  chapter.content = glassTideChapters[index];
});

export const mockCharacters: Character[] = [
  {
    id: "mara-vale",
    name: "Mara Vale",
    role: "Protagonist",
    age: 32,
    gender: "Woman",
    story: "glass-tide",
    tags: ["Observant", "Guarded", "Resilient"],
    image: "linear-gradient(160deg,#416172,#94aeb8 48%,#ddc9bc)",
    bio: "A mapmaker who charts changing coastlines and avoids the one shoreline she grew up on.",
    backstory:
      "Mara returned to her island hometown after inheriting her mother’s notebooks. The last pages hint at a tide that carries memories instead of shells.",
    relationships: [
      { name: "Elias Reed", relation: "Old friend; keeper of the lighthouse" },
      { name: "Nora Vale", relation: "Mother; an unfinished conversation" },
    ],
    dialogueCount: 48,
  },
  {
    id: "elias-reed",
    name: "Elias Reed",
    role: "Lighthouse keeper",
    age: 35,
    gender: "Man",
    story: "glass-tide",
    tags: ["Patient", "Loyal", "Secretive"],
    image: "linear-gradient(160deg,#314b59,#668c93 56%,#d2a995)",
    bio: "The keeper of the lighthouse at the edge of a coast that refuses to stay in place.",
    backstory:
      "Elias inherited the watch from his grandfather and knows why the bell rings on windless nights.",
    relationships: [
      { name: "Mara Vale", relation: "Childhood friend and unresolved first love" },
    ],
    dialogueCount: 32,
  },
  {
    id: "iris-bell",
    name: "Iris Bell",
    role: "Protagonist",
    age: 27,
    gender: "Woman",
    story: "midnight-orchard",
    tags: ["Curious", "Tender", "Determined"],
    image: "linear-gradient(160deg,#4b4059,#a9808f 58%,#edc7a6)",
    bio: "A botanist who comes home to settle an estate and finds an impossible winter orchard.",
    backstory:
      "Iris grew up hearing stories of trees that blossomed only in darkness. She never expected the stories to speak her name.",
    relationships: [
      { name: "June Bell", relation: "Grandmother; keeper of the ledger" },
    ],
    dialogueCount: 41,
  },
  {
    id: "june-bell",
    name: "June Bell",
    role: "The keeper",
    age: 73,
    gender: "Woman",
    story: "midnight-orchard",
    tags: ["Wise", "Wry", "Protective"],
    image: "linear-gradient(160deg,#465152,#a08f84 55%,#e3c8aa)",
    bio: "Iris’s grandmother left behind an orchard full of questions and a ledger full of clues.",
    backstory:
      "For fifty winters June tended the trees alone, protecting a bargain she had made long ago.",
    relationships: [{ name: "Iris Bell", relation: "Granddaughter and heir" }],
    dialogueCount: 23,
  },
  {
    id: "leonie-park",
    name: "Leonie Park",
    role: "Cartographer",
    age: 29,
    gender: "Woman",
    story: "atlas-of-small-worlds",
    tags: ["Inventive", "Restless", "Brave"],
    image: "linear-gradient(155deg,#2e536a,#76a1ae 52%,#d9c5ad)",
    bio: "An exacting cartographer who discovers that one of her father’s maps is a place.",
    backstory:
      "Leonie was raised among instruments, atlases, and the strange promise that every blank space has a reason.",
    relationships: [{ name: "Tomas Park", relation: "Father and first teacher" }],
    dialogueCount: 36,
  },
  {
    id: "sofia-morales",
    name: "Sofia Morales",
    role: "Musician",
    age: 34,
    gender: "Woman",
    story: "after-the-lanterns",
    tags: ["Expressive", "Stubborn", "Hopeful"],
    image: "linear-gradient(150deg,#604c63,#aa7b7b 52%,#e3b896)",
    bio: "A songwriter whose return home brings an unfinished song back into her life.",
    backstory:
      "Sofia left the city with one suitcase and a melody she could never quite complete.",
    relationships: [
      { name: "Theo Lin", relation: "Former bandmate and collaborator" },
    ],
    dialogueCount: 55,
  },
  {
    id: "theo-lin",
    name: "Theo Lin",
    role: "Composer",
    age: 36,
    gender: "Man",
    story: "after-the-lanterns",
    tags: ["Measured", "Kind", "Nostalgic"],
    image: "linear-gradient(150deg,#344e5f,#8495a3 52%,#c5a48d)",
    bio: "A composer who never stopped saving a place for Sofia in every new arrangement.",
    backstory:
      "Theo stayed when everyone else left, rebuilding the old rehearsal room one instrument at a time.",
    relationships: [
      { name: "Sofia Morales", relation: "Former bandmate; unresolved collaborator" },
    ],
    dialogueCount: 39,
  },
  {
    id: "nell-hart",
    name: "Nell Hart",
    role: "Student",
    age: 17,
    gender: "Girl",
    story: "paper-constellations",
    tags: ["Imaginative", "Quiet", "Loyal"],
    image: "linear-gradient(150deg,#3c4778,#8b90c1 55%,#e0b8ad)",
    bio: "A teenager who turns unsent letters into a night sky of possibilities.",
    backstory:
      "After moving schools, Nell started mapping the people she missed as stars she could still find.",
    relationships: [
      { name: "Ava Rowan", relation: "Best friend across two cities" },
    ],
    dialogueCount: 27,
  },
  {
    id: "ada-quill",
    name: "Ada Quill",
    role: "Archivist",
    age: 41,
    gender: "Woman",
    story: "the-quiet-archive",
    tags: ["Precise", "Skeptical", "Compassionate"],
    image: "linear-gradient(150deg,#39483f,#8b9888 55%,#d6baa2)",
    bio: "An archivist who believes every record deserves a witness, especially the ones people hide.",
    backstory:
      "Ada inherited the oldest library in town, and a collection of letters cataloged under a date that never existed.",
    relationships: [
      { name: "Evelyn Quill", relation: "Aunt; former curator of the archive" },
    ],
    dialogueCount: 46,
  },
];

const firstMessages = [
  {
    id: "msg-1",
    role: "user",
    content:
      "I want to open The Glass Tide with Mara returning to the shore after many years. Make it atmospheric, but keep the emotion grounded.",
    time: "10:24 AM",
  },
  {
    id: "msg-2",
    role: "assistant",
    content:
      "The ferry came in through a veil of morning rain, and Mara recognized the lighthouse before she recognized the town. Its bell sounded once, though the harbor was still. She kept her hands around the worn edge of her mother’s notebook and waited for the gangway to lower.",
    time: "10:24 AM",
  },
  {
    id: "msg-3",
    role: "user",
    content:
      "That’s lovely. Could you introduce the idea that the sea returns things people have forgotten?",
    time: "10:26 AM",
  },
  {
    id: "msg-4",
    role: "assistant",
    content:
      "At her feet, the water left a small blue button in the sand. Mara knew it at once: the missing button from the coat she had worn the winter her father taught her to read the tides. She had not thought about that coat in twenty years. She looked back at the sea as if it had spoken.",
    time: "10:26 AM",
  },
  {
    id: "msg-5",
    role: "user",
    content:
      "Now give Elias a quiet entrance. He should know more than he says.",
    time: "10:28 AM",
  },
  {
    id: "msg-6",
    role: "assistant",
    content:
      "“It does that,” Elias said behind her. He had grown into the lighthouse keeper’s coat as though he had always belonged to it. Mara closed her fingers around the button. He did not ask what she was holding, and somehow that told her he already knew.",
    time: "10:28 AM",
  },
];

export const mockChats: Chat[] = [
  {
    id: "chat-1",
    title: "Opening scene · The Glass Tide",
    projectId: "glass-tide",
    updatedAt: "Just now",
    messages: firstMessages,
  },
  {
    id: "chat-2",
    title: "The keeper’s confession",
    projectId: "glass-tide",
    updatedAt: "Yesterday",
    messages: [
      {
        id: "c2-1",
        role: "user",
        content:
          "Help me find a believable way for Elias to reveal what he has been hiding.",
        time: "Yesterday",
      },
      {
        id: "c2-2",
        role: "assistant",
        content:
          "Let him answer a small practical question first. When Mara asks why the bell rings without wind, he can admit he has been ringing it himself whenever the tide gives something back.",
        time: "Yesterday",
      },
    ],
  },
  {
    id: "chat-3",
    title: "Orchard chapter ideas",
    projectId: "midnight-orchard",
    updatedAt: "2 days ago",
    messages: [
      {
        id: "c3-1",
        role: "user",
        content: "What might Iris find among the trees after midnight?",
        time: "2 days ago",
      },
      {
        id: "c3-2",
        role: "assistant",
        content:
          "One pear bears a name she knows; another bears one she has never heard, written in the careful handwriting of her grandmother.",
        time: "2 days ago",
      },
    ],
  },
  {
    id: "chat-4",
    title: "A city between the pages",
    projectId: "atlas-of-small-worlds",
    updatedAt: "4 days ago",
    messages: [
      {
        id: "c4-1",
        role: "user",
        content: "Describe the moment Leonie steps inside the atlas.",
        time: "4 days ago",
      },
      {
        id: "c4-2",
        role: "assistant",
        content:
          "The page did not turn; the room did. Suddenly the paper had weather, and the inked streets held the warmth of late afternoon.",
        time: "4 days ago",
      },
    ],
  },
  {
    id: "chat-5",
    title: "Character voice · Sofia",
    projectId: "after-the-lanterns",
    updatedAt: "Last week",
    messages: [
      {
        id: "c5-1",
        role: "user",
        content: "What does Sofia say when she sees Theo again?",
        time: "Last week",
      },
      {
        id: "c5-2",
        role: "assistant",
        content:
          "“You kept the piano,” she says, because asking why he stayed would require admitting she wanted him to.",
        time: "Last week",
      },
    ],
  },
];
