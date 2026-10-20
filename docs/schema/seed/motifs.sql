-- ============================================================
-- MOTIFS SEED — rounds 1 and 2
-- Spec: docs/schema/system.dbml (SYSTEM_PROMPT.MOTIF / Table motifs)
-- Round 2: synthesis of seven independent web-search research assignments;
-- 20 additional candidates were reviewed for category fit and seed overlap.
-- Requirements: PostgreSQL 13+ (gen_random_uuid); idempotent by code.
-- All chapter bounds are NULL: motifs are optional story-planning anchors,
-- not prescribed placements. Authors choose applicable motifs.
-- ============================================================
-- Round 2 source index (descriptions are original paraphrases):
-- character_arc: Helping Writers Become Authors — corruption arc;
--   https://www.helpingwritersbecomeauthors.com/corruption-character-arc/
--   Britannica — hamartia: https://www.britannica.com/art/hamartia
--   EBSCO — alienation / identity crisis:
--   https://www.ebsco.com/research-starters/literature-and-writing/alienation-literary-theme/
--   https://www.ebsco.com/research-starters/literature-and-writing/identity-crisis-as-literary-theme/
-- relationship: Britannica — xenia / foil:
--   https://www.britannica.com/topic/xenia-sociology
--   https://www.britannica.com/art/foil-literature
--   Writer's Digest — friends-to-lovers:
--   https://www.writersdigest.com/write-better-fiction/how-to-write-a-friends-to-lovers-romance-when-youre-addicted-to-the-enemies-to-lovers-trope
--   Washington University thesis — sworn siblings:
--   https://repository.library.wustl.edu/cgi/viewcontent.cgi?article=1757&context=etd
--   CrimeReads — odd-couple partnerships:
--   https://crimereads.com/why-odd-couples-and-opposites-make-for-great-crime-fiction/
-- symbolic: peer-reviewed Jane Eyre mirror study:
--   https://scholarlypublishingcollective.org/psup/pcp/article/54/2/273/198651/All-the-Reflected-Light-We-Cannot-See-Ghastly
--   University of Michigan thesis on time in Gatsby:
--   https://lsa.umich.edu/content/dam/english-assets/migrated/honors_files/Caramagno,%20Natalie_Thesis.pdf
--   Secret Garden, chapter VII:
--   https://americanliterature.com/author/frances-hodgson-burnett/book/the-secret-garden/chapter-vii-the-key-to-the-garden
--   Hamlet flower discussion: https://study.com/learn/lesson/flower-symbolism-hamlet-william-shakespeare-overview-examples.html
--   Journey motif study: https://esciencepress.net/journals/IJES/article/download/2133/1038
-- plot: Britannica, Faustian bargain:
--   https://www.britannica.com/topic/Faustian-bargain
-- plot/symbolic folklore examples: Virgil, Georgics IV:
--   https://www.poetryintranslation.com/PITBR/Latin/VirgilGeorgicsIV.php
--   Ovid, Metamorphoses X:
--   https://www.poetryintranslation.com/PITBR/Latin/Metamorph10.php
--   Hospitality synthesis (educational source, not primary text):
--   https://www.learningtogive.org/units/generosity-spirit-folktales/hospitality
--   Research method/context: Harvard folklore index guide:
--   https://guides.library.harvard.edu/folk_and_myth/indices
-- Round 1 source index (round 1 seed rows are unchanged):
-- Aristotle, Poetics: https://classics.mit.edu/Aristotle/poetics.1.1.html
--   https://classics.mit.edu/Aristotle/poetics.2.2.html
-- Folger Shakespeare Library:
--   https://www.folger.edu/explore/shakespeares-works/romeo-and-juliet/
--   https://www.folger.edu/explore/shakespeares-works/
-- LiteraryDevices: https://literarydevices.net/motif/
--   https://literarydevices.net/symbolism/ https://literarydevices.net/foil/
-- Apollodorus, Library: https://www.theoi.com/Text/Apollodorus1.html
-- Homeric Hymns: https://www.theoi.com/Text/HomericHymns1.html
-- Sources support particular concepts/examples, not universal meanings.
-- ============================================================
BEGIN;

INSERT INTO motifs
  (id, code, name, category, version, is_system, description, min_chapter, max_chapter)
SELECT gen_random_uuid(), v.code, v.name, v.category, '1', TRUE, v.description, NULL, NULL
FROM (VALUES
  -- CHARACTER_ARC: change in understanding, values, identity, or agency.
  ('bildung_through_experience', 'Formation through experience', 'character_arc', 'Người trẻ hoặc thiếu kinh nghiệm được định hình qua các cuộc gặp gỡ, sai lầm, học hỏi và quan hệ, dần điều chỉnh nhận định và bản sắc của mình.'),
  ('self_recognition_after_evidence', 'Self-recognition', 'character_arc', 'Những bằng chứng tích lũy khiến nhân vật nhận ra một bản sắc, mối huyết thống, lịch sử hoặc sự thật về chính mình mà trước đó họ chưa hiểu.'),
  ('error_confronted_as_frailty', 'Confronting a consequential error', 'character_arc', 'Một sai lầm bắt nguồn từ giới hạn con người trở nên không thể né tránh; hành trình tập trung vào việc nhân vật nhận trách nhiệm hay phủ nhận nó.'),
  ('reversal_reveals_character', 'Reversal reveals character', 'character_arc', 'Một biến cố đảo chiều vận may phơi bày giá trị của nhân vật và thúc đẩy họ đổi hướng hành động.'),
  ('idealism_tempered_by_disillusionment', 'Idealism tested by experience', 'character_arc', 'Trải nghiệm mâu thuẫn với niềm tin lý tưởng hóa, buộc nhân vật xem xét lại thế giới quan mà không nhất thiết trở nên hoài nghi.'),
  ('complicity_to_moral_awareness', 'Moral awareness after complicity', 'character_arc', 'Nhân vật nhận ra vai trò của mình trong tổn hại gây ra và chuyển từ ngụy biện sang chịu trách nhiệm, có thể kèm thay đổi hành vi.'),
  ('loyalty_retested_by_conscience', 'Conscience over unquestioned loyalty', 'character_arc', 'Những nghĩa vụ xung đột khiến lòng trung thành vô điều kiện không còn đứng vững, buộc nhân vật cân nhắc lại trung thành thực sự đòi hỏi điều gì.'),
  ('fear_acknowledged_then_faced', 'Courage while afraid', 'character_arc', 'Nhân vật nhận diện nỗi sợ và vẫn chọn hành động có mục đích, thay vì phải trở nên không còn sợ hãi.'),
  ('dependence_to_self_direction', 'Movement toward self-direction', 'character_arc', 'Nhân vật chuyển từ dựa vào lựa chọn của người khác sang tự đưa ra và chịu trách nhiệm về các quyết định quan trọng.'),
  ('inherited_identity_reconsidered', 'Inherited identity reconsidered', 'character_arc', 'Nhân vật xem xét bản sắc hoặc vai trò được gia đình hay cộng đồng truyền lại, rồi chọn điều muốn giữ, thay đổi hoặc khước từ.'),
  ('grief_integrated_into_life', 'Grief integrated into life', 'character_arc', 'Nhân vật đưa mất mát vào ký ức và bản sắc của mình, và có thể gắn bó trở lại với cuộc sống mà không xóa bỏ hay giải quyết dứt điểm nỗi đau.'),
  ('trial_changes_the_seeker', 'Trial changes the seeker', 'character_arc', 'Những thử thách trong hành trình làm thay đổi nhận định hoặc cách hiểu bản thân của người tìm kiếm, khiến thành công không chỉ là giành được một vật phẩm.'),

  -- RELATIONSHIP: dynamics between characters; these are possibilities, not assignments.
  ('star_crossed_lovers', 'Star-crossed lovers', 'relationship', 'Hai người hình thành tình cảm lãng mạn có qua có lại, nhưng những thế lực ngoài tầm kiểm soát liên tục cản trở tương lai chung của họ.'),
  ('forbidden_romance', 'Forbidden romance', 'relationship', 'Tình yêu bị gia đình, luật pháp, địa vị, tập tục hoặc phe phái ngăn cấm, khiến cam kết đi kèm cái giá phải trả.'),
  ('enemies_to_lovers', 'From antagonism to romance', 'relationship', 'Các nhân vật khởi đầu trong thế đối địch rồi nảy sinh tình cảm lãng mạn nhờ hiểu nhau hơn, hợp tác hoặc hoàn cảnh thay đổi.'),
  ('unrequited_affection', 'Unreturned affection', 'relationship', 'Tình cảm lãng mạn hoặc gắn bó của một nhân vật không được đáp lại tương xứng, tạo nên nỗi mong chờ, hiểu lầm hoặc một lựa chọn.'),
  ('romantic_triangle', 'Romantic triangle', 'relationship', 'Những hấp dẫn hoặc cam kết chồng lấn tạo ra các lòng trung thành cạnh tranh, buộc nhân vật phải lựa chọn hoặc thương lượng lại quan hệ.'),
  ('rivals_to_allies', 'Rivals become allies', 'relationship', 'Các đối thủ nhận ra nhu cầu chung hoặc sự tôn trọng lẫn nhau, chuyển từ cạnh tranh được-mất sang hợp tác có điều kiện.'),
  ('mentor_apprentice_bond', 'Mentor and apprentice', 'relationship', 'Người dẫn dắt truyền đạt kiến thức hoặc năng lực cho người học; sự phụ thuộc có thể chuyển thành tự chủ, thách thức hoặc quan hệ tương hỗ.'),
  ('parent_child_estrangement', 'Parent-child estrangement', 'relationship', 'Cha mẹ và con cái bị chia cách bởi quyền uy, kỳ vọng, tổn thương hoặc khác biệt giá trị; quan hệ có thể được hàn gắn hoặc tiếp tục rạn nứt.'),
  ('sibling_rivalry', 'Sibling rivalry', 'relationship', 'Anh chị em cạnh tranh tình cảm, thừa kế, sự công nhận hoặc quyền lực, biến lịch sử chung thành nguồn gây áp lực và tổn thương.'),
  ('chosen_family', 'Chosen family', 'relationship', 'Các nhân vật xây dựng quan hệ gia đình bền chặt qua sự chăm sóc, trách nhiệm tương hỗ và trải nghiệm chung, ngoài hoặc bên cạnh quan hệ huyết thống.'),
  ('reluctant_alliance', 'Reluctant alliance', 'relationship', 'Những nhân vật ngờ vực hoặc đối lập hợp tác trước mối đe dọa chung; sự phụ thuộc thực tế có thể xuất hiện trước lòng tin.'),
  ('betrayal_and_repaired_trust', 'Betrayal and repaired trust', 'relationship', 'Một lần phản bội làm tổn hại quan hệ; việc hàn gắn đòi hỏi nhận trách nhiệm và thay đổi hành vi, chứ không mặc nhiên có sự tha thứ.'),

  -- SYMBOLIC: repeated images whose meanings are made by each work’s context.
  ('rain', 'Rain', 'symbolic', 'Mưa xuất hiện lặp lại có thể đi cùng cảm xúc mãnh liệt hoặc gợi sự gột rửa, khởi đầu mới; ý nghĩa tùy thuộc bối cảnh tác phẩm.'),
  ('light_darkness', 'Light and darkness', 'symbolic', 'Sự tương phản lặp lại giữa ánh sáng và bóng tối có thể gợi hy vọng, tri thức, bất định hoặc tuyệt vọng trong câu chuyện.'),
  ('red_color', 'Red imagery', 'symbolic', 'Hình ảnh màu đỏ lặp lại có thể gắn với đam mê, giận dữ, hiểm nguy hoặc tình yêu trong một tác phẩm cụ thể.'),
  ('masks_and_facades', 'Masks and façades', 'symbolic', 'Việc đeo mặt nạ theo nghĩa đen hoặc ẩn dụ lặp lại có thể khai thác bản sắc bị che giấu, sự lừa dối hoặc đối lập giữa vẻ ngoài và thực chất.'),
  ('birds_and_captivity', 'Birds and captivity', 'symbolic', 'Hình ảnh chim lặp lại có thể gợi liên tưởng đến tự do, giam hãm hoặc kháng cự, tùy theo bối cảnh câu chuyện.'),
  ('watching_eyes', 'Watching eyes', 'symbolic', 'Những đôi mắt hoặc ánh nhìn lặp lại có thể gợi sự giám sát hoặc chứng kiến về mặt đạo đức; câu chuyện tự xác lập hàm ý đó.'),
  ('storms_and_weather', 'Storms and weather', 'symbolic', 'Thời tiết lặp lại có thể phản chiếu hoặc tương phản với trạng thái cảm xúc của nhân vật, thay vì chỉ làm phông nền.'),
  ('water', 'Water', 'symbolic', 'Nước xuất hiện lặp lại có thể gợi sự sống, tái sinh, thanh tẩy hoặc tiềm thức trong hệ thống biểu tượng riêng của tác phẩm.'),
  ('fire', 'Fire', 'symbolic', 'Lửa xuất hiện lặp lại có thể gợi đam mê, hủy diệt, biến đổi hoặc thanh lọc trong một câu chuyện cụ thể.'),
  ('forest', 'Forest', 'symbolic', 'Hình ảnh hoặc bối cảnh rừng lặp lại có thể gợi điều chưa biết, hiểm nguy hoặc một hành trình hướng nội.'),
  ('distant_green_light', 'Distant green light', 'symbolic', 'Một ánh sáng xanh ở phía xa có thể biểu trưng cho khát vọng chưa đạt tới, như trong The Great Gatsby; đây là cách diễn giải đặc thù theo tác phẩm.'),
  ('conch_shell', 'Conch shell', 'symbolic', 'Vỏ ốc tù và xuất hiện lặp lại có thể tượng trưng cho trật tự hoặc quyền uy dân chủ trong một tác phẩm, như Lord of the Flies.'),

  -- PLOT: reusable event patterns, not fixed formulas.
  ('quest_for_object', 'Quest for an object or goal', 'plot', 'Một hành trình theo đuổi mục tiêu quan trọng vượt qua nhiều trở ngại và làm thay đổi điều người tìm kiếm hiểu hoặc sẵn lòng hy sinh.'),
  ('revenge_pursuit', 'Pursuit of revenge', 'plot', 'Nhân vật bị tổn hại theo đuổi sự trả đũa, phơi bày cái giá đạo đức hoặc vòng lặp tiếp diễn của tổn thương.'),
  ('mistaken_identity', 'Mistaken identity', 'plot', 'Sự nhầm lẫn về danh tính của một người dẫn đến những cuộc gặp và rắc rối quan trọng trước khi sự thật được làm sáng tỏ.'),
  ('marriage_test', 'Test for partnership', 'plot', 'Nhân vật chính phải vượt qua một điều kiện hoặc thử thách để đạt được hôn nhân hay sự kết đôi; câu chuyện tự quyết định mức độ công bằng và hệ quả của thử thách.'),
  ('false_accusation', 'False accusation', 'plot', 'Lời nói dối hoặc sự hiểu sai khiến người vô tội bị xem là có tội, làm tổn hại tình hình trước khi sự thật có thể được công nhận.'),
  ('crime_unmasking', 'Unmasking a concealed crime', 'plot', 'Người điều tra tái dựng một tội ác bị che giấu và xác định trách nhiệm dựa trên chứng cứ cùng suy luận.'),
  ('ambition_usurpation', 'Ambition and usurpation', 'plot', 'Nỗ lực giành quyền lực lật đổ trật tự hiện có, kéo theo sự kháng cự, hậu quả hoặc sụp đổ.'),
  ('family_feud', 'Family feud', 'plot', 'Sự thù địch giữa các dòng họ hạn chế lựa chọn cá nhân và duy trì xung đột cho đến khi có đổ vỡ hoặc hòa giải.'),
  ('return_and_reckoning', 'Return and reckoning', 'plot', 'Một người vắng mặt hoặc bị lưu lạc trở về nơi ở hay cộng đồng đã đổi khác, rồi đối diện hậu quả của sự vắng mặt hoặc hành động trong quá khứ.'),
  ('survival_escape', 'Survival and escape', 'plot', 'Những người mắc kẹt trong hiểm nguy tìm cách thoát thân; các nỗ lực thử thách sự tháo vát và làm thay đổi thứ tự ưu tiên giữa sinh tồn với lòng trung thành.'),
  ('prophecy_fulfillment', 'Prophecy and fulfillment', 'plot', 'Một lời tiên tri chi phối quyết định và được ứng nghiệm qua hành động, dù nhân vật chống lại hay vô tình khiến nó thành sự thật.'),
  ('impossible_task_quest', 'Impossible-task quest', 'plot', 'Nhân vật chính phải hoàn thành một nhiệm vụ tưởng như bất khả thi để được thông hành, công nhận hoặc nhận một vật phẩm; nhiệm vụ có thể công bằng, bị ép buộc hoặc là cái bẫy.'),

  -- ROUND 2 CHARACTER_ARC candidates: adjacent concepts retained with narrower definitions.
  ('moral_corruption', 'Moral corruption', 'character_arc', 'Nhân vật ngày càng chấp nhận và biện minh cho những hành vi hoặc giá trị khiến chuẩn mực đạo đức của mình suy thoái.'),
  ('error_driven_downfall', 'Error-driven downfall', 'character_arc', 'Một sai lầm nghiêm trọng trong phán đoán trở thành động lực dẫn đến sự suy sụp của nhân vật; sai lầm đó không nhất thiết là lỗi đạo đức.'),
  ('alienation', 'Alienation', 'character_arc', 'Nhân vật trải qua hoặc ngày càng xa cách về cảm xúc hay tâm lý với xã hội hoặc môi trường xung quanh; không mặc định rằng sự xa cách sẽ được hóa giải.'),
  ('identity_reckoning', 'Identity reckoning', 'character_arc', 'Nhân vật đối diện mâu thuẫn giữa cách tự nhìn nhận bản thân và kỳ vọng xã hội, rồi tìm cách xác định hoặc thể hiện mình.'),

  -- ROUND 2 RELATIONSHIP candidates.
  ('guest_friendship_xenia', 'Guest-friendship and hospitality', 'relationship', 'Quan hệ chủ–khách được định hình bởi sự tiếp đón, bảo vệ và nghĩa vụ có đi có lại, đôi khi vượt ra ngoài một lần gặp gỡ.'),
  ('friends_to_lovers', 'Friends to lovers', 'relationship', 'Một tình bạn thuần túy đã hình thành chuyển thành tình yêu lãng mạn có qua có lại, đưa lịch sử chung và lòng tin vào một kiểu quan hệ mới.'),
  ('sworn_siblinghood', 'Sworn siblinghood', 'relationship', 'Những người không cùng huyết thống thề kết nghĩa anh chị em và nhận lấy lòng trung thành cùng nghĩa vụ tương hỗ như người thân, đôi khi qua nghi lễ hoặc lời thề chính thức.'),
  ('contrastive_foil_pair', 'Contrastive foil pair', 'relationship', 'Những khác biệt có ý nghĩa giữa các nhân vật giúp làm rõ phẩm chất, giá trị hoặc lựa chọn của nhau; họ không nhất thiết là đối thủ.'),
  ('odd_couple_partnership', 'Odd-couple partnership', 'relationship', 'Một mối cộng tác kéo dài giữa những nhân vật có tính khí hoặc thế giới quan khác biệt rõ rệt, trong đó sự tương phản tạo nên căng thẳng hoặc bổ trợ lẫn nhau.'),

  -- ROUND 2 SYMBOLIC candidates: readings depend on each work’s context.
  ('mirrors_reflections', 'Mirrors and reflections', 'symbolic', 'Hình ảnh phản chiếu lặp lại có thể gợi cách nhân vật nhìn nhận bản thân, sự xa lạ hoặc hiện tượng nhân đôi; ý nghĩa được xác lập qua cảnh truyện và cách kể.'),
  ('clocks_timepieces', 'Clocks and timepieces', 'symbolic', 'Đồng hồ hoặc vật đo thời gian xuất hiện lặp lại có thể nhấn mạnh trải nghiệm thời gian của nhân vật, chẳng hạn căng thẳng giữa hành động hiện tại với quá khứ được nhớ lại hoặc theo đuổi.'),
  ('keys_and_locks', 'Keys and locks', 'symbolic', 'Chìa khóa hoặc ổ khóa lặp lại có thể đánh dấu quyền tiếp cận một không gian bị hạn chế hoặc sự thay đổi trong những gì nhân vật có thể chạm tới; ý nghĩa tùy thuộc cốt truyện.'),
  ('flowers_and_plants', 'Flowers and plants', 'symbolic', 'Hoa và thực vật được nhắc đến hoặc xuất hiện có chủ ý nhiều lần có thể mang liên tưởng riêng theo từng cảnh; không mặc định một ý nghĩa biểu tượng cố định.'),
  ('journey_paths', 'Journeys and paths', 'symbolic', 'Hình ảnh hành trình hoặc lối đi lặp lại có thể tổ chức sự thay đổi, chia cách hoặc những cuộc gặp gỡ; cách diễn giải cần dựa vào bối cảnh câu chuyện.'),

  -- ROUND 2 PLOT candidates: reusable patterns, not prescribed formulas.
  ('prohibition_broken_on_return', 'Forbidden condition during return', 'plot', 'Việc trở về hoặc được thông hành phụ thuộc vào một điều kiện rõ ràng; vi phạm điều kiện ấy khiến nhân vật mất kết quả đã được trao, như trong một số dị bản chuyện Orpheus và Eurydice.'),
  ('shape_shifting_to_evade_capture', 'Transformation to evade capture', 'plot', 'Một nhân vật bị bắt giữ hoặc khống chế liên tục biến đổi hình dạng để tìm cách trốn thoát, cho đến khi người bắt giữ giữ được họ.'),
  ('prayer_animates_artwork', 'Artwork made living', 'plot', 'Một hình tượng được tạo tác trở nên sống động nhờ ân huệ thần linh hoặc nghi thức, biến hình ảnh hay vật thể được tưởng tượng thành một sự hiện diện sống.'),
  ('hospitality_limits_tested', 'Hospitality tested by travelers', 'plot', 'Cuộc gặp với khách lữ hành cho thấy chủ nhà ứng xử ra sao trước những kỳ vọng và giới hạn của lòng hiếu khách; các dị bản có thể khác nhau giữa những truyền thống.'),
  ('faustian_bargain', 'Faustian bargain', 'plot', 'Nhân vật giao kèo với một thế lực mà họ biết là tà ác hoặc vô đạo đức để đổi lấy lợi ích phi thường, chấp nhận cái giá có thể hủy hoại chính mình.'),
  ('grief_becomes_named_plant', 'Grief embodied in a plant', 'symbolic', 'Câu chuyện gắn nỗi tang thương hoặc mất mát với một loài cây qua sự biến hóa, khiến hình ảnh sinh vật ấy trở thành dấu tích đau buồn mang ý nghĩa riêng trong tác phẩm.'),

  -- ROUND 3: 57 MOTIFS BO SUNG TU 60 DE XUAT ANIME (CHARACTER_ARC & RELATIONSHIP)
  ('reluctant_messiah', 'Đấng cứu thế bất đắc dĩ', 'character_arc', 'Nhân vật bị đẩy vào vai trò cứu thế hoặc người hòa giải bởi hoàn cảnh và lời tiên tri, phải giằng xé với gánh nặng tâm lý và sự kỳ vọng quá mức của tập thể.'),
  ('vengeance_as_identity', 'Trả thù như bản sắc tồn tại', 'character_arc', 'Bản sắc và ý nghĩa tồn tại của nhân vật bị đồng nhất hoàn toàn với việc báo thù, khiến bản ngã tan rã và mất phương hướng khi mục tiêu biến mất.'),
  ('sacrificial_villainization', 'Tự biến mình thành ác nhân để hy sinh', 'character_arc', 'Nhân vật cố ý chấp nhận để bản thân bị hiểu lầm và nguyền rủa như một kẻ ác nhằm bảo vệ đại cục, lý tưởng hoặc người mình yêu thương.'),
  ('survivor_guilt_as_teacher', 'Người dẫn dắt mang mặc cảm sống sót', 'relationship', 'Mối quan hệ dẫn dắt mà người thầy xây dựng vai trò hướng đạo như một nỗ lực chuộc lỗi và đền tội cho những người trong quá khứ họ không thể cứu.'),
  ('trauma_certainty', 'Niềm tin tuyệt đối sinh ra từ chấn thương', 'character_arc', 'Nhân vật biến nỗi đau và bi kịch cá nhân thành xác tín tuyệt đối rằng chỉ có đau thương cực độ mới cứu rỗi hay thức tỉnh được nhân loại, không còn khả năng tự phản tư.'),
  ('freedom_as_responsibility', 'Tự do đi liền với trách nhiệm', 'character_arc', 'Nhân vật theo đuổi tự do tuyệt đối nhưng liên tục đối mặt với sức nặng trách nhiệm đối với những người tin tưởng đi theo mình, nhận ra tự do thực thụ luôn gắn liền với nghĩa vụ.'),
  ('vow_to_the_dead', 'Lời thề với người đã khuất', 'relationship', 'Mối liên kết tâm linh với một người đã chết định hình toàn bộ chuẩn mực sống, lý tưởng danh dự và lòng trung thành của nhân vật đối với người còn sống.'),
  ('pride_driven_allegiance', 'Đồng minh vì kiêu hãnh', 'relationship', 'Mối quan hệ liên minh được thúc đẩy bởi lòng kiêu hãnh cá nhân và nhu cầu vượt qua đối thủ hơn là sự tin cậy chân thành hay nguy cơ chung thuần túy.'),
  ('combat_hunger', 'Khát vọng chiến đấu thuần túy', 'character_arc', 'Nhân vật được định hình và trưởng thành chủ yếu thông qua ham muốn đối đầu với những giới hạn và kẻ mạnh hơn, biến chiến đấu thành bản năng sinh tồn tinh thần.'),
  ('benevolent_tyranny', 'Chuyên chế nhân danh điều tốt', 'relationship', 'Mối quan hệ áp đặt giải pháp lên người khác nhân danh lợi ích tối thượng của họ, biến sự hy sinh cá nhân thành quyền lực tước đoạt quyền tự quyết của tha nhân.'),
  ('god_complex_escalation', 'Leo thang phức cảm tự thần thánh hóa', 'character_arc', 'Nhân vật bắt đầu với lý tưởng thanh lọc công lý nhưng từng bước leo thang thành ảo tưởng quyền năng thần thánh, trong đó mỗi tội ác mới được hợp thức hóa bằng tội ác trước.'),
  ('dream_as_tyranny', 'Giấc mơ trở thành bạo quyền', 'character_arc', 'Khát vọng cá nhân vĩ đại biến thành chế độ chuyên chế tinh thần, biến mọi sinh mạng và bạn đồng hành thành công cụ tiêu hao phục vụ cho đỉnh cao danh vọng.'),
  ('trauma_bonding', 'Gắn kết qua chia sẻ chấn thương', 'relationship', 'Mối liên kết sâu sắc hình thành từ việc cùng trải qua, chịu đựng hoặc chứng kiến những tổn thương thể xác và tinh thần tột cùng hơn là tình cảm êm đềm thông thường.'),
  ('loneliness_of_the_strongest', 'Nỗi cô độc của kẻ mạnh nhất', 'character_arc', 'Sức mạnh vô song tạo ra khoảng cách bản thể học không thể san lấp với đồng loại, khiến nhân vật bị cô lập cấu trúc ngay giữa cộng đồng mình bảo vệ.'),
  ('humanitarian_radicalization', 'Cực đoan hóa từ lòng trắc ẩn', 'character_arc', 'Sự tha hóa bắt nguồn từ chính lòng trắc ẩn với nỗi đau của đồng loại, biến mong muốn bảo vệ kẻ yếu thành logic bạo lực diệt trừ triệt để nguồn cơn đe dọa.'),
  ('sadistic_curiosity', 'Hiếu kỳ tàn độc phi công cụ', 'character_arc', 'Nhân vật hành động tàn nhẫn không vì quyền lực hay bi kịch quá khứ mà xuất phát từ sự tò mò thực nghiệm bản năng về giới hạn chịu đựng và bản chất linh hồn con người.'),
  ('intellectual_condescension', 'Khinh miệt trí tuệ có hệ thống', 'relationship', 'Mối quan hệ bất đối xứng mà nhân vật xem người khác thuần túy như những mẫu vật thí nghiệm hoặc quân cờ giải trí do niềm tin tuyệt đối vào sự vượt trội trí tuệ của mình.'),
  ('resentment_engine', 'Động cơ từ oán hận giai tầng', 'character_arc', 'Mọi tham vọng và hành vi tàn bạo của nhân vật được vận hành bởi mối hận thù nguồn cội và mặc cảm xuất thân, thúc đẩy nỗ lực đè bẹp trật tự cũ để chứng minh bản thân.'),
  ('nihilistic_charisma', 'Mê hoặc hư vô mang tính hủy diệt', 'relationship', 'Sức hút cá nhân ma mị được dùng để lột trần sự trống rỗng và tha hóa ý chí sinh tồn của người khác, đẩy đối phương vào tự hủy hoại.'),
  ('hybrid_identity_negotiation', 'Giằng xé bản sắc lai tạp', 'character_arc', 'Nhân vật bị kẹt giữa hai giống loài hoặc cộng đồng bài trừ lẫn nhau, liên tục phải đàm phán và chịu đựng nỗi đau giằng xé để định nghĩa bản thân.'),
  ('protective_manipulation', 'Thao túng nhằm bảo bọc', 'relationship', 'Mối quan hệ mà người bề trên dẫn dắt người dưới bằng sự tàn nhẫn, dối trá và thao túng tâm lý nhưng với mục đích sâu xa là rèn luyện cho họ khả năng sống sót.'),
  ('fractured_self_reconstruction', 'Tái thiết bản thể phân mảnh', 'character_arc', 'Nhân vật bị phân liệt nhân cách và khủng hoảng niềm tin vào danh tính thực, nỗ lực tìm kiếm sự gắn kết bản ngã thông qua lòng trung thành tuyệt đối với một nhóm.'),
  ('immortal_regret', 'Hối tiếc muộn màng của kẻ trường sinh', 'character_arc', 'Nhân vật bất tử chỉ thấu hiểu sâu sắc giá trị của tình cảm và thời gian sau khi những người phàm trần đồng hành đã qua đời, biến ký ức thành hành trình thấu cảm muộn.'),
  ('atonement_through_service', 'Chuộc lỗi bằng chính tài năng gây họa', 'character_arc', 'Nhân vật dùng chính kỹ năng hoặc tri thức từng gây ra sai lầm cấm kỵ để phụng sự tha nhân, biến năng lực thành phương tiện đền tội và trả giá.'),
  ('institutional_guilt', 'Mặc cảm tội lỗi hệ thống', 'character_arc', 'Nhân vật gánh chịu sự dằn vặt vì đã tham gia vào bộ máy bạo lực tập thể, phấn đấu vươn lên đỉnh cao quyền lực để chịu trách nhiệm và cải tổ hệ thống từ bên trong.'),
  ('past_as_gravity', 'Quá khứ như lực hút tất yếu', 'character_arc', 'Quá khứ không chỉ là ký ức mà hoạt động như một định mệnh có sức nặng cấu trúc, kéo ghì nhân vật trở lại giải quyết món nợ sinh tử bất chấp nỗ lực rũ bỏ.'),
  ('pacifism_as_burden', 'Gánh nặng của lý tưởng bất bạo động', 'character_arc', 'Nguyên tắc tuyệt đối không sát sinh không đem lại sự thanh thản mà trở thành gông cùm đạo đức nghiệt ngã, đòi hỏi nhân vật phải trả giá bằng chính máu và nỗi đau của mình.'),
  ('retired_killer_penance', 'Sám hối của sát thủ gác kiếm', 'character_arc', 'Nhân vật mang quá khứ nhuốm máu xây dựng đạo đức mới trên nền tảng của sự kiềm chế tuyệt đối và lời thề bảo vệ kẻ yếu để bù đắp cho những sinh mạng đã cướp đi.'),
  ('protective_rage', 'Thịnh nộ bảo vệ bộc phát', 'character_arc', 'Sức mạnh và sự biến chuyển tâm lý của nhân vật được kích hoạt bởi cơn giận dữ khi chứng kiến người thân bị tổn thương, biến bạo lực thành phương tiện che chở.'),
  ('deconditioning', 'Tháo gỡ điều kiện hóa cưỡng bức', 'character_arc', 'Hành trình nhân vật từng bước phá vỡ các cài đặt tâm lý sợ hãi, bạo lực và phục tùng do gia đình hoặc môi trường áp đặt từ thơ ấu để giành lấy quyền tự quyết.'),
  ('innocence_as_cruelty', 'Sự ngây thơ tàn nhẫn', 'character_arc', 'Lòng thuần khiết và cách nhìn nhận thế giới đơn cực của nhân vật trở thành nguồn cơn của sự nhẫn tâm vô thức và hủy diệt khi bị đẩy vào nghịch cảnh cùng cực.'),
  ('emergent_humanity', 'Nhân tính trỗi dậy ở sinh thể phi nhân', 'character_arc', 'Một sinh thể vốn sinh ra với bản tính săn mồi và thống trị dần đánh thức lương tri, tình cảm và sự trân trọng sự sống thông qua mối tương tác chân thành với một con người.'),
  ('base_desire_heroism', 'Chủ nghĩa anh hùng từ dục vọng bản năng', 'character_arc', 'Nhân vật hành động trượng nghĩa không xuất phát từ đại nghĩa hay lý tưởng cao đẹp mà từ những nhu cầu thể xác cơ bản và khát khao hơi ấm con người bình dị.'),
  ('fraudulent_mentorship', 'Người dẫn dắt giả tạo nhưng chân tình', 'relationship', 'Mối quan hệ hướng đạo mà người thầy không hề có năng lực chuyên môn thực sự nhưng lại trao đi sự thấu hiểu tâm lý, tình cảm chân thành và bảo bọc đạo đức thiết yếu.'),
  ('love_as_obligation', 'Tình yêu gắn chặt với nghĩa vụ', 'relationship', 'Tình cảm lãng mạn bị hòa lẫn và trói buộc với lời hứa sinh tồn và nghĩa vụ bảo vệ, khiến sự gắn bó vừa là nơi nương tựa vừa là gông xiềng tâm lý giằng xé.'),
  ('freedom_through_destruction', 'Tìm tự do qua hủy diệt tận cùng', 'character_arc', 'Nhân vật đi đến kết luận rằng tự do tuyệt đối chỉ có thể đạt được khi xóa bỏ toàn bộ thế giới áp bức bên ngoài, biến bạo lực hủy diệt thành hành động giải phóng cực đoan.'),
  ('genocidal_compassion', 'Lòng trắc ẩn mang tính diệt chủng', 'character_arc', 'Nhân vật ôm giữ niềm tin méo mó rằng chấm dứt sự tồn tại của chính chủng tộc mình là hành vi nhân đạo tối cao để giải thoát họ vĩnh viễn khỏi nỗi thống khổ trần gian.'),
  ('split_loyalty_psychosis', 'Tâm thần phân liệt vì xung đột trung thành', 'character_arc', 'Nhân vật phát triển các nhân cách mâu thuẫn để đối phó với sự giằng xé đạo đức không thể hòa giải khi vừa đóng vai người anh em trung thành vừa là kẻ thù gián điệp.'),
  ('time_loop_obsession', 'Ám ảnh cố định qua vòng lặp thời gian', 'relationship', 'Mối quan hệ bị biến dạng khi một bên liên tục lặp lại quá khứ để cứu người kia, biến sự quan tâm thuần khiết thành nỗi ám ảnh cô độc, kiểm soát và xa cách thực tế.'),
  ('systemic_self_sacrifice', 'Hy sinh thân mình để tái lập hệ thống', 'character_arc', 'Hành vi từ bỏ sự tồn tại cá nhân không phải để cứu một cá nhân riêng lẻ mà để viết lại luật lệ của toàn bộ một trật tự vũ trụ bất công và cứu rỗi tập thể.'),
  ('artificial_personhood', 'Quá trình hình thành nhân vị nhân tạo', 'character_arc', 'Một sinh thể nhân tạo vốn chỉ được xem là công cụ dần thức tỉnh ý niệm về bản ngã, linh hồn và quyền được sở hữu cảm xúc độc lập thông qua các va chạm kết nối.'),
  ('overcompensation_identity', 'Bản sắc xây dựng từ sự bù trừ tâm lý', 'character_arc', 'Nhân vật dựng lên một vỏ bọc kiêu ngạo, xuất chúng và bất cần để che giấu nỗi sợ kinh hoàng về việc bị bỏ rơi và sự vô giá trị bên trong tâm hồn.'),
  ('avoidant_agency', 'Chủ động bằng cách né tránh', 'character_arc', 'Sự trưởng thành của nhân vật diễn ra thông qua các chu kỳ thoái lui và từ chối hành động, cho đến khi nhận ra việc trốn chạy cũng là một lựa chọn mang hệ quả tất yếu.'),
  ('posthuman_identity_quest', 'Truy vấn bản ngã hậu nhân loại', 'character_arc', 'Hành trình triết học kiếm tìm ranh giới của bản ngã và ý thức khi thể xác và ký ức có thể bị số hóa, thay thế và phân mảnh hoàn toàn vào mạng lưới dữ liệu.'),
  ('network_dissolution', 'Hòa tan bản ngã vào mạng lưới kết nối', 'character_arc', 'Ranh giới giữa cái tôi cá nhân và hệ thống thông tin toàn cầu dần bị xóa nhòa, khiến nhân vật tiến hóa từ một con người đơn lẻ thành một thực thể phổ quát vô hình.'),
  ('curiosity_as_defiance', 'Tính hiếu kỳ như sự phản kháng thân phận', 'character_arc', 'Nhân vật dùng niềm đam mê tri thức và óc quan sát sắc bén để vượt qua những lề thói ngột ngạt và nguy hiểm của địa vị xã hội thấp kém mà không cần phô trương quyền lực.'),
  ('sibling_as_cause', 'Tình thân vừa là động lực vừa là giới hạn', 'relationship', 'Mối quan hệ ruột thịt đóng vai trò là ngọn nguồn khởi xướng mọi mưu đồ chính trị vĩ mô, đồng thời là ranh giới đạo đức duy nhất giữ cho nhân vật không hoàn toàn mất nhân tính.'),
  ('love_as_competition', 'Tình yêu như trò chơi quyền lực', 'relationship', 'Mối quan hệ tình cảm mà hai bên xem việc bộc lộ cảm xúc trước là sự đầu hàng, biến những tương tác lãng mạn thành những ván cờ tâm lý cân não vì lòng tự ái.'),
  ('cynical_idealism', 'Chủ nghĩa lý tưởng khoác áo hoài nghi', 'character_arc', 'Nhân vật dùng thái độ cay nghiệt và các giải pháp tự hy sinh thể diện để bảo vệ một khát vọng ngây thơ về những mối quan hệ chân thực không vụ lợi.'),
  ('emotional_literacy', 'Học chữ cái của cảm xúc', 'character_arc', 'Một công cụ chiến tranh học cách trở thành con người thông qua việc thu nhận ngôn từ, thấu cảm nỗi đau và diễn đạt tình cảm của người khác qua những bức thư.'),
  ('innocent_conspirator', 'Đồng phạm ngây thơ', 'relationship', 'Mối liên kết mà đứa trẻ nắm giữ bí mật của người lớn và tham gia vào các kế hoạch hiểm hóc với sự hồn nhiên trẻ thơ, tạo nên sự ấm áp và hài hước tình huống.'),
  ('performative_family', 'Gia đình nhập vai hóa thực tế', 'relationship', 'Mối quan hệ gia đình khởi đầu từ một thỏa thuận ngụy trang vì nhiệm vụ thực dụng, nhưng qua sinh hoạt hàng ngày sự gắn kết giả lập dần hóa thành tình thân ruột thịt.'),
  ('dual_life_intimacy', 'Thân mật trong sự che giấu thân phận', 'relationship', 'Mối quan hệ mà sự gắn bó và ấm êm chỉ có thể duy trì nhờ việc mỗi bên bảo vệ và giấu kín phần tăm tối chết chóc nhất trong danh tính của mình.'),
  ('fatalistic_partnership', 'Cộng sự dưới bóng định mệnh', 'relationship', 'Mối quan hệ đồng hành giữa những con người cùng ngầm hiểu rằng sự gắn bó này chỉ là tạm thời và mỗi người đều đang chờ đợi thời khắc quay về với định mệnh tất yếu của mình.'),
  ('improvised_brotherhood', 'Huynh đệ ngẫu hứng qua va chạm', 'relationship', 'Mối dây liên kết bằng hữu gắn bó keo sơn được tôi luyện thuần túy qua chuỗi ngày kề vai sinh tử và phiêu bạt ngẫu nhiên mà không cần đến bất kỳ lời thề ước trang trọng nào.'),
  ('trauma_clowning', 'Hề hóa nỗi đau chiến tranh', 'character_arc', 'Nhân vật khoác lên mình vẻ lười biếng, bỡn cợt và bất cần đời như một cơ chế tự vệ tâm lý kiên cố để sống chung với những bóng ma và tổn thương chiến tranh.'),
  ('powerless_omnipotence', 'Sự bất lực của kẻ toàn năng', 'character_arc', 'Trạng thái khủng hoảng tâm lý và lãnh cảm cùng cực khi đạt đến đỉnh cao tuyệt đối không còn đối thủ, biến sức mạnh vô song thành sự tước đoạt toàn bộ niềm vui phấn đấu.')
) AS v(code, name, category, description)
ON CONFLICT (code) DO NOTHING;

COMMIT;
