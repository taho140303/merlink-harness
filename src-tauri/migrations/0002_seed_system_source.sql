-- 0002_seed_system_source.sql - system catalog and prompt seed data.
--
-- Consolidated baseline for Merlink Harness. Squashed from the previous 26 forward-only
-- migration files before the first public release; see CONTRIBUTING.md ("Database
-- migrations") for the policy that governs new migrations.

-- Applied on top of 0001_schema.sql. The references below are foreign-key deferred so
-- the seed can be loaded in catalog order inside a single transaction.
PRAGMA defer_foreign_keys = ON;


-- Idempotent seed for standard reference data (e.g. system genres)
INSERT OR IGNORE INTO genres (id, code, display_name, description) VALUES
    ('018e3a00-0000-7000-8000-000000000001', 'fantasy', 'Fantasy', 'Fantasy genre and subgenres'),
    ('018e3a00-0000-7000-8000-000000000002', 'sci_fi', 'Science Fiction', 'Science Fiction genre and subgenres'),
    ('018e3a00-0000-7000-8000-000000000003', 'mystery', 'Mystery', 'Mystery and detective fiction');

-- Migration 0004: seed system style presets for databases already at v3.
-- This is data-only and safe to reapply due to stable IDs and INSERT OR IGNORE.

-- System style presets from idea/system_prompt/style_presets (example.json excluded).
-- Stable IDs and codes keep reapplying this seed idempotent.
-- The DBML has no fields for name, settings, detailed enum qualifiers, rhetorical
-- intensity/examples, or rules; those source fields are not represented here.
-- Free-text pace, imagery, and syntax values are reduced to their leading enum
-- level; parenthetical detail and mixed-range nuance cannot be stored in DBML.
-- Preset codes use the source relative path without .json.
INSERT OR IGNORE INTO rhetorical_devices (id, code, description) VALUES
    ('019a6500-0000-79c0-8d63-800c095ad6f2', 'style_d067e70d3906f6b7809ee41b', 'Bi kịch hóa'),
    ('019a6500-0000-74d0-9920-aee8a42af8ed', 'style_c26fcdd9d7aac1b12a0a5263', 'Câu chấm lửng hụt hơi'),
    ('019a6500-0000-7a0f-92f9-998ff032e9e4', 'style_10959ad59591627df61e682a', 'Câu chấm lửng nghẹn'),
    ('019a6500-0000-7e63-8b4d-82def9bb78a1', 'style_4896131a4fdae388ba7d4b00', 'Câu chấm lửng nghẹn lại'),
    ('019a6500-0000-7033-87b3-8c1c42560558', 'style_8dd07d1f2a578d9cf8d008f3', 'Câu chấm lửng nghẹn ngào'),
    ('019a6500-0000-7800-818a-a74a60184fbc', 'style_902833defdc3d5dab8401658', 'Câu chữ dài dòng'),
    ('019a6500-0000-77a1-b887-4498c8deba56', 'style_a23bb2c95c0267c494cc02f7', 'Câu chữ lủng củng'),
    ('019a6500-0000-74ec-bf94-928e80cafa03', 'style_66369360e9d09fc4ca7f5ded', 'Câu chữ đứt đoạn hỗn loạn'),
    ('019a6500-0000-7974-a0b2-5ef14378c55c', 'style_e279e0a331065a7fba6dba3f', 'Câu cáo buộc lặp & câu hỏi tu từ sắc bén'),
    ('019a6500-0000-7ff9-8343-09607f3edc8d', 'style_9a2b8242af9eba73ca4e8a26', 'Câu cảm thán bùng nổ'),
    ('019a6500-0000-70c1-ad10-f694eef3b333', 'style_8c4ddced1284aaa21beb8d02', 'Câu cảm thán hào hứng'),
    ('019a6500-0000-767e-8215-4fbda2ebf0a5', 'style_c405be9ee524d72fc3232353', 'Câu cảm thán hãi hùng'),
    ('019a6500-0000-7ba1-bc4a-dc9970bb4782', 'style_61ab49d0ab57d918a4c15ad1', 'Câu cảm thán ngắn'),
    ('019a6500-0000-7383-a78a-9ffc0972bef9', 'style_575d77cbed044767415b3cda', 'Câu cụt cộc lốc'),
    ('019a6500-0000-7ab6-b59a-3c1c6adce03e', 'style_11be98d2dccf0370006c0225', 'Câu cụt dứt khoát'),
    ('019a6500-0000-7686-b935-8c49f6ac5aa2', 'style_4a5a8e2426e3adae4dd350f8', 'Câu cụt hỗn loạn không dấu phẩy'),
    ('019a6500-0000-7801-ae1b-9635fa0a0570', 'style_442dccd44e890f12e47d7300', 'Câu cụt liên tiếp'),
    ('019a6500-0000-7baf-bc4d-240cd0896f11', 'style_0394335ebc9ac62d7505af39', 'Câu cụt liên tiếp không liên từ'),
    ('019a6500-0000-70aa-b4a6-1fdcd1c9c075', 'style_99196871eba7f31b2b09691f', 'Câu cụt thiếu chủ ngữ'),
    ('019a6500-0000-76ff-bcb3-40fcd39d3627', 'style_df3cf6d29cecfa007edd28d1', 'Câu cụt vô cảm'),
    ('019a6500-0000-7653-a9f8-db4c74855ed5', 'style_a6a7dda88995b0f71b3d9f63', 'Câu dài'),
    ('019a6500-0000-7ea0-be76-4da7b8eb3e26', 'style_686ca33589eb324eb5fe8733', 'Câu ghép dài'),
    ('019a6500-0000-7ba9-8bfd-7da9e1603fd9', 'style_eeda9654ed72e31a985e400e', 'Câu ghép dài mang tính thống kê'),
    ('019a6500-0000-714e-aa2b-2238dbf1096a', 'style_0512d347e2f88c7bb35d9bbe', 'Câu giật gân'),
    ('019a6500-0000-7efa-bbcb-6f1c1faef619', 'style_efb789463b71ae9c6d27ec41', 'Câu hỏi quan tâm'),
    ('019a6500-0000-7616-876c-b27bd9fa9425', 'style_e3e4ecb149afd158b752b867', 'Câu hỏi tu từ'),
    ('019a6500-0000-7614-bd96-aadcf55d8aaf', 'style_16bd7ee30580d89e35e97adf', 'Câu hỏi tu từ bộc phát'),
    ('019a6500-0000-7d52-8aac-f29adccecd12', 'style_c37602fc5c4d8f3e2a8f41c9', 'Câu hỏi tu từ căng thẳng'),
    ('019a6500-0000-755b-a3b4-50b7cf07f5b9', 'style_9e89205d89c737551c767df6', 'Câu hỏi tu từ gây áp lực'),
    ('019a6500-0000-752e-8417-50ba8c10f3e9', 'style_b9e4fc154e47ebe26291225f', 'Câu hỏi tu từ kinh ngạc'),
    ('019a6500-0000-76a1-a961-f91d4a3b5c3a', 'style_f28664738bde7bba2df08f60', 'Câu hỏi tu từ lo lắng'),
    ('019a6500-0000-7bee-8955-add413976b01', 'style_152af7d3e0691e8f3b6dcb24', 'Câu hỏi tu từ mỉa mai'),
    ('019a6500-0000-73ba-940e-09f83c92c170', 'style_d2a5c3c8cc0ee91b57b7b162', 'Câu hỏi tu từ thăm dò'),
    ('019a6500-0000-717f-a624-d4c7f3aee3eb', 'style_03b90a04f5164a61ccfb1cc5', 'Câu hỏi tu từ thầm thì trong suy nghĩ'),
    ('019a6500-0000-778e-aa7b-a8c4aa6746f9', 'style_7b7645a9148bced3baee3b6c', 'Câu hỏi tu từ về bản thân & ẩn dụ lốt/thực chất'),
    ('019a6500-0000-7cef-99e1-e12cd76fb65e', 'style_c304fd9ce4b3eb0c0de6e483', 'Câu hỏi tu từ về quá khứ'),
    ('019a6500-0000-7f2e-a428-fc87be692303', 'style_6a15fe87551ec81dd210a99f', 'Câu hỏi tu từ về số phận'),
    ('019a6500-0000-781e-a563-a124ad5088b8', 'style_5af3be79b01635864864be6e', 'Câu hỏi tu từ về định mệnh'),
    ('019a6500-0000-7790-b4f3-dc7c853664aa', 'style_1aaa9ecabd81604b345234a7', 'Câu khẳng định dứt khoát'),
    ('019a6500-0000-7051-a1bd-41a5252dc467', 'style_3d69079e289bc7f0e5700da3', 'Câu lửng hoài niệm'),
    ('019a6500-0000-7e8e-99ca-6e5bf7fac5c0', 'style_fc40340f019d76f60b6328c7', 'Câu rút gọn chủ ngữ'),
    ('019a6500-0000-71e8-8c0c-8cf537f03cca', 'style_8874357a871548bb9f11e8c3', 'Câu rút gọn cảnh báo'),
    ('019a6500-0000-70dd-b584-bf394e6f192a', 'style_0ca72e478677cb25c4c253e2', 'Câu trang trọng khô khan'),
    ('019a6500-0000-7092-8f60-f91bef16014e', 'style_cf1c9aefd51a60d064807bc2', 'Cảm thán giận dữ'),
    ('019a6500-0000-7d61-978a-a0f2f9ccdf1d', 'style_7a539b5fe6366ea8a56beb49', 'Cảm xúc cá nhân'),
    ('019a6500-0000-792f-aadf-6b8d04a243fd', 'style_740f9d21fe80dd90c70929d3', 'Dòng ý thức lặp đi lặp lại'),
    ('019a6500-0000-781e-890c-26a2ab1e860a', 'style_7e823acaea04573389df1dab', 'Hài hước'),
    ('019a6500-0000-77db-9d36-34bfc90a5ad4', 'style_bd4646bb81c5a5e5b0b699a2', 'Hành động dồn dập'),
    ('019a6500-0000-790a-9bf2-e7382c29f001', 'style_ab38fe82aaa21cfa1535d625', 'Kể sự kiện khô khan'),
    ('019a6500-0000-7f4b-a765-a4e22e69eed0', 'style_6cd20c7eabf328ebbc2d7539', 'Kịch tính'),
    ('019a6500-0000-72aa-a894-cc6618eb0fc8', 'style_48933ad35fa23c07e68d0fee', 'Liệt kê biểu tượng'),
    ('019a6500-0000-727a-a588-19eb0d4700b0', 'style_ceb065f42b7ca4fa475e2b88', 'Liệt kê biểu tượng quyền lực'),
    ('019a6500-0000-7dc2-a55e-77b5297c31c0', 'style_d00ee2008dbe238bb6720da9', 'Liệt kê chi tiết kỳ lạ'),
    ('019a6500-0000-7a72-ac21-8b8d9a91da38', 'style_43c164e0693139c6d0091d7e', 'Liệt kê chướng ngại'),
    ('019a6500-0000-74cc-b180-e34db3d32996', 'style_0fc1b0111197f226f5ebf03f', 'Liệt kê cổ vật'),
    ('019a6500-0000-759c-867f-4b9a76d1f38a', 'style_a6c595390a028afc74212e4b', 'Liệt kê dấu hiệu mong manh'),
    ('019a6500-0000-762c-a457-94949052a544', 'style_22db01fa90b9c66d0b4c3d4d', 'Liệt kê dồn dập & câu cụt liên tiếp (tạo hiệu ứng ngộp thở)'),
    ('019a6500-0000-723d-9c43-e13f52e4f239', 'style_774e5af2ba9d1991df07a07e', 'Liệt kê hiểu lầm chồng chất'),
    ('019a6500-0000-788f-8a70-7b8ae6c72622', 'style_e5876a62910eccae17e4a73c', 'Liệt kê hành vi nhập vai'),
    ('019a6500-0000-7c63-8500-0fdd86243288', 'style_4dbb2fb8f927a86e39b70e3e', 'Liệt kê hướng dẫn di chuyển (rẽ, tránh, lướt)'),
    ('019a6500-0000-7317-b7bf-73baaa278964', 'style_6713fc14cf3e86f3626cd08c', 'Liệt kê khoảnh khắc chuyển giao'),
    ('019a6500-0000-7c66-8422-3502a14f8b4f', 'style_68684e5cced7f7fd5132d50d', 'Liệt kê thói quen lặp lại & điệp nhịp êm đềm'),
    ('019a6500-0000-79b8-9063-0360456599ee', 'style_b82fcffa47a1989bf7fc3ce3', 'Liệt kê toàn cảnh & điệp cấu trúc so sánh lực lượng'),
    ('019a6500-0000-796b-a7e6-dc3a8b21f237', 'style_273771c30765283e468b4974', 'Liệt kê trình tự & câu mệnh lệnh'),
    ('019a6500-0000-7a4c-9287-15371c179746', 'style_17ae53bb7a2e395f1037898d', 'Liệt kê điều khoản'),
    ('019a6500-0000-7635-88b4-c34e3d379270', 'style_f720b4a26d4ee5c50d6cb31c', 'Lạc quan thái quá'),
    ('019a6500-0000-7012-946c-98b79419c78f', 'style_91cff4f42eb6a47ef1ccd849', 'Lặp cấu trúc điều kiện (Nếu... thì...)'),
    ('019a6500-0000-79d1-979a-fe05d541579d', 'style_e2d47e10f5d44df5c5df31d8', 'Miêu tả chi tiết máy móc'),
    ('019a6500-0000-750d-8c11-f474ca697159', 'style_02fc595e11d12c67fae9c5e4', 'Miêu tả chi tiết từng ngày'),
    ('019a6500-0000-7381-9fb2-e6a796aa8c44', 'style_38be165daca4eb8b76587acd', 'Miêu tả chậm chạp'),
    ('019a6500-0000-7ed7-ad1a-704c8e39e19e', 'style_1b09455f18f8442fb57f19f3', 'Miêu tả khô khan'),
    ('019a6500-0000-7fd7-a705-19bd85dc15fe', 'style_7a8e720bf13858d5ff7ae97a', 'Miêu tả ngoại cảnh dài'),
    ('019a6500-0000-73b7-8ce3-3bcfebe079f4', 'style_0a403ddcc9f155afb31d1093', 'Miêu tả nội tâm dài'),
    ('019a6500-0000-7bb4-9c73-7a85962812d1', 'style_8404cb603c52549f9fb6520c', 'Miêu tả tĩnh lặng'),
    ('019a6500-0000-7991-89af-47817eac2c08', 'style_e54dd2ac8d4aafbfb211833a', 'Miêu tả tĩnh lặng kéo dài'),
    ('019a6500-0000-73a0-a576-5ee77b7a275e', 'style_30aa181a0209322be69460f9', 'Mạch lạc logic'),
    ('019a6500-0000-7ebb-8ebe-884d9eface7e', 'style_65c6feb4dd648d9cdaecc95e', 'Mỉa mai châm biếm'),
    ('019a6500-0000-715c-aa74-1c31b40d1b93', 'style_c7f9de0082a1a80b62ccd5a9', 'Ngôn ngữ bi kịch'),
    ('019a6500-0000-7092-abfe-14836d6072fb', 'style_7bb5df259e52c8ac05b5bed3', 'Ngôn ngữ bi quan'),
    ('019a6500-0000-71b9-8047-12e84bdfd553', 'style_25f425dbfdde85344c29480c', 'Ngôn ngữ buồn thảm'),
    ('019a6500-0000-7291-a348-e4da007ee88f', 'style_dddd8961239c5634964d3df6', 'Ngôn ngữ bình thản'),
    ('019a6500-0000-7695-ac8b-827aaf2926fe', 'style_0a04353ee7e48256e004d2a6', 'Ngôn ngữ bình tĩnh'),
    ('019a6500-0000-7932-b4de-f7ff7361994a', 'style_9d5b00bcd5fa1386515b8a16', 'Ngôn ngữ chậm chạp, nặng nề'),
    ('019a6500-0000-795d-9167-8746858887fc', 'style_8a8edf41799f8f4934b40a55', 'Ngôn ngữ chậm rãi'),
    ('019a6500-0000-7525-866d-ac34a4fb1ed7', 'style_0455ed9f47ae0ae185b8c065', 'Ngôn ngữ căng thẳng'),
    ('019a6500-0000-7d16-ab7a-15287341dca8', 'style_1075889116a6e06f2aa64ad8', 'Ngôn ngữ cảm xúc'),
    ('019a6500-0000-723c-8281-214be6d86343', 'style_75c274540b76d7e7b130b8be', 'Ngôn ngữ cảm xúc cá nhân'),
    ('019a6500-0000-76cd-9161-d28eae5790b2', 'style_2c63a42131624c8ec19d94b0', 'Ngôn ngữ cảm xúc mạnh'),
    ('019a6500-0000-747d-a391-ab757ed1a948', 'style_9be537c607989ef974a99507', 'Ngôn ngữ cảm xúc thái quá'),
    ('019a6500-0000-781a-b79e-8088aaf4e3f3', 'style_4a4d01cf44fc24c7f951a7c7', 'Ngôn ngữ gấp gáp'),
    ('019a6500-0000-7ecb-8525-3f9475e02197', 'style_909a2690b9ac7a0922ecb694', 'Ngôn ngữ hiện đại, tục'),
    ('019a6500-0000-7920-8c38-16fc7d7483a0', 'style_9e0a25421c1682a54af0721d', 'Ngôn ngữ hài hước'),
    ('019a6500-0000-779f-8f43-43a21dd73716', 'style_eafafaa19bdf53228abb7263', 'Ngôn ngữ khô khan, lạnh lùng'),
    ('019a6500-0000-7bd8-9379-afcdc7cab0d1', 'style_caf6edfdd60bc146c54e3e5e', 'Ngôn ngữ mơ hồ, do dự'),
    ('019a6500-0000-7234-aa25-b7b08ebc8bd5', 'style_dca1751422a7a0666b09e20a', 'Ngôn ngữ mỉa mai'),
    ('019a6500-0000-75c2-b1cb-090fce6a7b1b', 'style_96de0323e488cc9b12fb7c5c', 'Ngôn ngữ nghiêm túc'),
    ('019a6500-0000-7e5e-826a-48afc36611c4', 'style_73668809eb56244208e66299', 'Ngôn ngữ quá đỗi thường'),
    ('019a6500-0000-77c5-8918-0ded7d2229c4', 'style_ff6fffb3fb7a2fd454a3dd7c', 'Ngôn ngữ rụt rè'),
    ('019a6500-0000-7e39-9fcc-6762a51560bb', 'style_6f32bdfaac93d7111170681c', 'Ngôn ngữ thông tục ngoài lề'),
    ('019a6500-0000-7f72-a81b-1785504e542f', 'style_75e5b919df24b79f16bae6ea', 'Ngôn ngữ thẳng thừng'),
    ('019a6500-0000-79cb-ab2b-aed1a742aadb', 'style_a7af5cf3a13b2d160137e35f', 'Ngôn ngữ thờ ơ'),
    ('019a6500-0000-7b4c-b4c4-42533de734d1', 'style_c56553867e615205cd83ce3b', 'Ngôn ngữ tuyệt vọng tuyệt đối'),
    ('019a6500-0000-7766-a2b8-f6660979a21c', 'style_bdac52e0663615fa6e9e0a77', 'Ngôn ngữ u ám'),
    ('019a6500-0000-7b07-85bc-61da2a3e7fbe', 'style_aee975b272dccf4ae46d30cf', 'Ngôn ngữ vui vẻ'),
    ('019a6500-0000-7093-a8d8-10192b3dda77', 'style_379778ef9545edfd11097f58', 'Ngôn ngữ yếu ớt'),
    ('019a6500-0000-7c30-a504-026fbecda593', 'style_6dcbb686af13c3ea34ad4eb4', 'Ngôn ngữ đời thường'),
    ('019a6500-0000-7fd8-9745-b9fbc131d658', 'style_9cc4cdbb8f5fdb45c45b8248', 'Ngôn ngữ ẩn dụ mơ hồ'),
    ('019a6500-0000-76e2-88ac-2ba412e16fcb', 'style_d4b51379dba601fe1c3012e5', 'Ngôn ngữ ồn ào'),
    ('019a6500-0000-7e2f-b6e7-9db2793291a6', 'style_4a639dbe93e52f32f9b4e1d8', 'Nhịp văn chậm'),
    ('019a6500-0000-7bc0-95b4-fd072f2c7804', 'style_0ff17c360fc23ba9fbe463e3', 'Nhịp điệu trầm đều'),
    ('019a6500-0000-7de0-8a98-cb44b6b00fa1', 'style_ff57a5513b9188eb893979b9', 'Phép liệt kê & đối chiếu thời gian'),
    ('019a6500-0000-70d0-8e97-910219bdf08c', 'style_9467233854de59f1bf4798fa', 'Phép liệt kê chi tiết & nhấn chi tiết bất thường'),
    ('019a6500-0000-7e0a-a038-2e1b80dd6db4', 'style_f65c7e61412f7b57a9e33bbb', 'Phép liệt kê chặng đường & điệp cấu trúc cắt cảnh'),
    ('019a6500-0000-707c-9d2f-f7d50ac00db8', 'style_2ddac3ec55c3433c534a4457', 'Phép liệt kê chứng cứ'),
    ('019a6500-0000-7fb3-9032-db7f95f2602e', 'style_5e4e6e1fcce3acc56aa842a3', 'Phép liệt kê công đoạn & điệp cấu trúc nâng cấp'),
    ('019a6500-0000-76b9-b225-0c7828a58e99', 'style_e914cb9feb5aa4a7e2dd6e37', 'Phép liệt kê manh mối & điệp cấu trúc đuổi theo'),
    ('019a6500-0000-7264-95a7-1169d9fb67ae', 'style_e09d9bca6d15bdea70661916', 'Phép liệt kê mã lệnh'),
    ('019a6500-0000-7579-82c1-621043f650a4', 'style_3aba7750895e821c8bd75045', 'Phép liệt kê món ăn & điệp nhịp vui nhộn'),
    ('019a6500-0000-7385-8484-6525b267e3f9', 'style_74c6585b8a69be284f3e9915', 'Phép liệt kê món đồ & đối thoại mặc cả lặp'),
    ('019a6500-0000-7bbb-9d02-205760a24b9b', 'style_ffbc470ca47104e87b6b52a2', 'Phép liệt kê mùa / tháng & phép đối trước – sau'),
    ('019a6500-0000-72b8-96ee-46aa0592bf6f', 'style_8402346cec81c89da96c7dc0', 'Phép liệt kê nguyện vọng'),
    ('019a6500-0000-7179-9ccf-eadf87255938', 'style_06b852f793555c194afe0f51', 'Phép liệt kê sự thật'),
    ('019a6500-0000-7db0-85fd-f27632a56efc', 'style_d4772cce3574760693f42fad', 'Phép liệt kê trò vui & điệp nhịp hội'),
    ('019a6500-0000-756b-a916-3451692b9c56', 'style_e8785123a40ed11925178c7f', 'Phép liệt kê đòn đánh'),
    ('019a6500-0000-7477-be0f-45765040c99b', 'style_8e189994e76083967f5f5391', 'Phép liệt kê địa danh & điệp cấu trúc khai mở'),
    ('019a6500-0000-711d-bc9a-90f122a339b8', 'style_a58cd32090da8dc977788bf0', 'Phép loại suy'),
    ('019a6500-0000-77b9-aec6-aef234a11cc3', 'style_1dc27c7e4ea76f2b19f703e1', 'Phép lặp câu hỏi đối chất & liệt kê chứng cứ'),
    ('019a6500-0000-7e8a-8bb8-c33928b2cf88', 'style_080116442e6fa3873a15f496', 'Phép phóng đại sức mạnh & điệp cấu trúc vượt giới hạn'),
    ('019a6500-0000-7707-ada0-3831f1bc72b6', 'style_871f96833fba6fd5fd470e77', 'Phép phóng đại sự mất kiểm soát'),
    ('019a6500-0000-78a6-9d70-87ac7dcab939', 'style_463fec65f56f776be3af81fe', 'Phép phóng đại trớ trêu & điệp cấu trúc dây chuyền'),
    ('019a6500-0000-7bda-aee3-4a395fc553cc', 'style_2a1898e26e195614dea47b6c', 'Phép so sánh lạ – quen & ẩn dụ khám phá'),
    ('019a6500-0000-73ae-8da5-274da7ae0461', 'style_8140342e64949dc66ca90b44', 'Phép so sánh mâu thuẫn'),
    ('019a6500-0000-7b3e-a378-c688047944a6', 'style_07b9f2d1f3bcdf24ed8097aa', 'Phép so sánh trước – sau'),
    ('019a6500-0000-7a50-ac6c-1db4014c81ad', 'style_2235d02fb6f63598a3ad0577', 'Phép tam đoạn luận & điệp cấu trúc suy luận'),
    ('019a6500-0000-7975-b0c8-fa67924f7a9e', 'style_e9b45b2a8334bb5d338f2ae6', 'Phép điệp câu hỏi dồn dập & lặp ý gài bẫy'),
    ('019a6500-0000-79bf-a94c-99712132f284', 'style_dfb019d27c711444e82c731c', 'Phép điệp toàn cảnh lặp lại & phép đối giữa các lần thử'),
    ('019a6500-0000-782b-a4ff-663b90f37366', 'style_63ace1f2e98b50cdc0fba88a', 'Phép đảo ngữ & nhấn ''hoá ra / thì ra'''),
    ('019a6500-0000-70c4-a6ec-4cbd7af2a03e', 'style_1970b9442852dc683346c649', 'Phép đối & liệt kê hỗn tạp'),
    ('019a6500-0000-7d01-a3f0-f9e3e95c3b32', 'style_e9e4388213fd437e5d055dcf', 'Phép đối ''anh ta – tôi'''),
    ('019a6500-0000-71c9-8e2d-d2060a870ed7', 'style_9ae3fd64482318ce76d20ad7', 'Phép đối ''trước – sau'' theo thời gian'),
    ('019a6500-0000-7fa2-87f7-e6c19227ed93', 'style_39f28bc61ad1f4206e83c8e4', 'Phép đối ''tôi đã tin – hoá ra'' & điệp khoảng lặng'),
    ('019a6500-0000-7eab-bb3f-3cb033412f55', 'style_df207c0a60de23986157ed1d', 'Phép đối (cảnh - người, quá khứ - hiện tại, bên trong - bên ngoài)'),
    ('019a6500-0000-7317-9c40-209b67a5857d', 'style_d387026be17594248977f23b', 'Phép đối buông tay – nắm giữ'),
    ('019a6500-0000-773a-a988-5f92c6e793a3', 'style_a0cc5413f034bcf2a37055ef', 'Phép đối buộc tội – bào chữa'),
    ('019a6500-0000-7eb7-a61f-a342921a5972', 'style_ee17cad9a9103ba9e8d0ed2e', 'Phép đối dấu vết'),
    ('019a6500-0000-73ee-86fd-cd492f75dc9e', 'style_db56cc7a42680406c148af84', 'Phép đối giữa các phần của bản thân'),
    ('019a6500-0000-72f3-88aa-c5e1bf59858c', 'style_69d00c63b38930c18554a650', 'Phép đối giữa hai nhân vật'),
    ('019a6500-0000-78f7-9fff-482f1d039cfe', 'style_f8e5d995050bfeebe349bb77', 'Phép đối giữa hai đối thủ'),
    ('019a6500-0000-7f6e-956b-88e6ef323226', 'style_eed7220e0e9387d236cd2dfd', 'Phép đối giữa hỏi và tránh'),
    ('019a6500-0000-703f-998c-bf1a68ed07d5', 'style_d5f1a3da77670ae6d2d0c9f8', 'Phép đối giữa thân phận giả và thật'),
    ('019a6500-0000-717d-80ad-bdff81a502ce', 'style_4d7143a223ce860adc16e600', 'Phép đối giữa đường đi và cản trở'),
    ('019a6500-0000-7eaa-9ff1-ddfff78c9536', 'style_36248e5470f285d9349b2f85', 'Phép đối hai phe & lặp điều kiện để nhấn'),
    ('019a6500-0000-7529-bfe2-5b70678c22b5', 'style_5757aafd63e6da3740ca28de', 'Phép đối hôm nay – ngày mai'),
    ('019a6500-0000-77a8-87ef-188948620694', 'style_5e7885f0df05509d774af305', 'Phép đối khoảng lặng và lời nói'),
    ('019a6500-0000-7177-a425-f1423b712446', 'style_41665dddecf6e7b529dbf52c', 'Phép đối ký ức – hiện tại'),
    ('019a6500-0000-7e21-af35-40d6ecae3d03', 'style_aee62d4d4ed65d4e17564de5', 'Phép đối kẻ trị – người trị'),
    ('019a6500-0000-77c7-b144-e5221bd81ced', 'style_ed9905eb949e85ee0cb92892', 'Phép đối lý trí – cảm xúc'),
    ('019a6500-0000-70b5-ac95-a4005892dff1', 'style_00c62b75aa5b41e82fff5996', 'Phép đối lời thề'),
    ('019a6500-0000-7c5e-a0bb-d6ffec4a2bbc', 'style_48dc0d2da1ab522bd5f81282', 'Phép đối màu sắc, âm thanh'),
    ('019a6500-0000-7160-826d-55e31a0e380a', 'style_b61abbcd0779cf5ffa55df7a', 'Phép đối người bán – người mua'),
    ('019a6500-0000-7048-8313-fe6e101763a7', 'style_bbf4ada9e329a1211eb75757', 'Phép đối người thắng – kẻ thua'),
    ('019a6500-0000-74da-8cc6-2f202b5b8514', 'style_2b4eede64052cbc128f8a52c', 'Phép đối tranh cãi – hòa giải vui vẻ'),
    ('019a6500-0000-7cbf-81c9-4dedb4256f4d', 'style_9ca3a12ac69351741ef589c4', 'Phép đối trung thành – phản bội'),
    ('019a6500-0000-7293-b9b3-11c456ba790b', 'style_1db49cd60500c896a6909b2d', 'Phép đối trước – sau'),
    ('019a6500-0000-7c7a-9bee-3a426a776c5b', 'style_1626de25a661ef6e31c7a841', 'Phép đối trước – sau khi lưu đày'),
    ('019a6500-0000-78e3-95b8-16b71a83b966', 'style_9c36056b2190f740dcb79204', 'Phép đối tương phản hài hước'),
    ('019a6500-0000-71b6-8a7c-936e14ca3b57', 'style_c9e2f6f077161c37affbb2ac', 'Phép đối vùng đất này – vùng đất kia'),
    ('019a6500-0000-775c-a6b2-136efc2fa6cc', 'style_a7aaf9ee2502fbcb0279969d', 'Phóng đại'),
    ('019a6500-0000-7d43-8e6d-06b04e1ae196', 'style_09dd1e0fb6b05ceeb52eac99', 'Phóng đại kịch tính'),
    ('019a6500-0000-7a68-84ca-ebb25cce8bf3', 'style_7df640484367b27c8d27ac0e', 'So sánh chuyển động chính xác'),
    ('019a6500-0000-7bcf-a9e9-625a3b6edcd1', 'style_24c3b6325ce94835b7a1313c', 'So sánh cảm giác'),
    ('019a6500-0000-76c8-9e89-5a7faaed7d8d', 'style_556324a748569f1fcaac58ec', 'So sánh dài dòng'),
    ('019a6500-0000-7355-a878-c6681553add7', 'style_3761a581dd5ea727052bddb9', 'So sánh không gian mạng – thực địa'),
    ('019a6500-0000-7cb2-8028-aba4eb88cc60', 'style_f51735df4f78a7879c634e70', 'So sánh ngầm (Ẩn dụ chuyển đổi cảm giác)'),
    ('019a6500-0000-7970-bb95-4dfab5295df0', 'style_fa53c2d9f0db303b202c3b53', 'So sánh với ký ức & điệp câu nói dang dở'),
    ('019a6500-0000-7490-b42c-ca18bdbd29cc', 'style_27f08faf795ca072dd8b267e', 'So sánh đối chiếu sửa sai'),
    ('019a6500-0000-740b-96f0-50921ecd7c37', 'style_1ac77344ca79920b55e8a94e', 'Suy tư nội tâm dài'),
    ('019a6500-0000-7789-b89d-3eafb6e2ef4e', 'style_2bc364b6a159dffdb08ab9e3', 'Trào phúng'),
    ('019a6500-0000-7f73-81b7-cf42ad2e4b95', 'style_9aaf15a4d2791926e4331afa', 'Tượng thanh chế tác'),
    ('019a6500-0000-7e6f-a355-6fdfd90579e1', 'style_41ed7464e6b9ff23fda50bd1', 'Tượng thanh chợ búa'),
    ('019a6500-0000-7f61-b2aa-acc298af9b34', 'style_ff12706d74d27bffb56a0f96', 'Tượng thanh dồn dập'),
    ('019a6500-0000-7457-b4d9-7321de1e7161', 'style_dc12fe3051cd31e129048e56', 'Tượng thanh hài kịch'),
    ('019a6500-0000-785f-8d1a-d0deea475375', 'style_b1a260b9f4dbf962017e6597', 'Tượng thanh hỗn loạn'),
    ('019a6500-0000-7bcd-b2bf-7aacf1e3bb07', 'style_66976d565528a00aa7e1f8d7', 'Tượng thanh nhẹ nhàng'),
    ('019a6500-0000-7072-acc4-e79f692a556d', 'style_96036d1706890caac7685e41', 'Tượng thanh nhịp bước'),
    ('019a6500-0000-7b82-804a-c59bc156d87f', 'style_28f0a25b29d7a6dd664c18b0', 'Tượng thanh nhịp di chuyển'),
    ('019a6500-0000-7817-a97e-c9b004979770', 'style_9e30ba169da0423cb0d36ade', 'Tượng thanh nhịp độ & điệp khúc ''xa/gần'''),
    ('019a6500-0000-770b-886b-5f9802bdf404', 'style_2fab2f21cc70079e2794024a', 'Tượng thanh náo nhiệt'),
    ('019a6500-0000-7b38-902f-f90232096656', 'style_78f95078d95c9282e6d612e4', 'Tượng thanh tiếng động đuổi theo'),
    ('019a6500-0000-7acb-8d28-4d519b04e698', 'style_a3231ac647d8755f3cd38ef5', 'Tượng thanh ám ảnh & điệp hình ảnh quá khứ xâm nhập'),
    ('019a6500-0000-77fc-b583-31e15c3f8de2', 'style_93276c6332b4b3dbebb018b8', 'Tượng thanh ồn'),
    ('019a6500-0000-7446-ab65-5c315d456cf9', 'style_cc0113241a9efc97c7a1d9f2', 'Tượng thanh ồn ào'),
    ('019a6500-0000-78d8-a97d-1beb0bb22289', 'style_1f6acc5eba5a4809f1bb1169', 'Tập trung vào cá nhân'),
    ('019a6500-0000-7a67-9d23-d8a2fa5ca5f3', 'style_09e6480c7f36a04f440ec867', 'Từ ngữ hành động hớn hở'),
    ('019a6500-0000-7b48-9405-5f2550fd04da', 'style_2bfbcadfb62b3c2085206a38', 'Điệp cấu trúc ''chưa...'' & ẩn dụ thời gian chậm lại'),
    ('019a6500-0000-7407-9a6c-561005a85ff9', 'style_41ecfed8133c12a904468c9b', 'Điệp cấu trúc bước tiến & liệt kê cột mốc'),
    ('019a6500-0000-70fe-8cec-d5668478fa16', 'style_a9f07d12f8f5d211da401675', 'Điệp cấu trúc câu cụt mơ hồ'),
    ('019a6500-0000-7b00-8e00-1f670cbb5290', 'style_430a72cefc995ea543ecff1e', 'Điệp cấu trúc kêu gọi & câu hỏi tu từ thuyết phục'),
    ('019a6500-0000-7317-aec5-af6abea2b18e', 'style_f5017eeca29ba09b9241ad4d', 'Điệp cấu trúc phân mảnh'),
    ('019a6500-0000-7f34-b05c-24a60eb8807f', 'style_4a9d4100c93c6ad5de06b687', 'Điệp cấu trúc quan sát'),
    ('019a6500-0000-7b1a-b429-784133dcba7f', 'style_f23a936eace84068a4817b78', 'Điệp cấu trúc quan sát cảnh giác'),
    ('019a6500-0000-7266-b506-1ba0dec19e57', 'style_c0633e9b8bb4e4712845321d', 'Điệp cấu trúc quá đều đặn'),
    ('019a6500-0000-7b43-a3a4-40cb5bcdbd65', 'style_b29905a5a8ecc9047bc29a37', 'Điệp cấu trúc thần chú & ẩn dụ ranh giới thế giới'),
    ('019a6500-0000-7aea-a555-e1fb819b7433', 'style_5b94f77fc4c248ad422db027', 'Điệp cấu trúc thế đứng & nhịp thở'),
    ('019a6500-0000-7b17-96a2-e0b9e49654cc', 'style_a20d4801f18dbe8d2ad6b8a2', 'Điệp cấu trúc trao đổi nửa chừng'),
    ('019a6500-0000-78f8-8142-f3042f1dfc16', 'style_3f0c255c3737ed1b6bddad68', 'Điệp cấu trúc tuyên thệ & ẩn dụ vương miện nặng trĩu'),
    ('019a6500-0000-7c5e-9ebf-a61924c8631d', 'style_890be7a9f2e9abe71c4824bb', 'Điệp cấu trúc tấn công từng lớp & ẩn dụ đột nhập'),
    ('019a6500-0000-7f9f-99e2-51f61d214a74', 'style_2626a8b59cc5a6b17b7cb1e5', 'Điệp cấu trúc vui vẻ & liệt kê điều nhỏ nhặt'),
    ('019a6500-0000-7ff0-9baf-11d6d4a3feb4', 'style_99f1134e968a3ecf575e0e4e', 'Điệp cấu trúc xác minh'),
    ('019a6500-0000-7ea5-8d76-ff5c4a8f4609', 'style_4dea15de40f46add1a96d5d4', 'Điệp giọng cao dồn dập'),
    ('019a6500-0000-76eb-a55b-02bd96a748dd', 'style_d9009e2e23b27449d0fff3e6', 'Điệp từ'),
    ('019a6500-0000-739e-ac77-8a8bbf40e77e', 'style_c8afd301295b2a11a11a12fd', 'Điệp từ chỉ số lượng đám đông'),
    ('019a6500-0000-7ffb-bc8e-24fc5386c19c', 'style_5e1ecb6fbc4b4e52a1c3c8cb', 'Điệp từ chỉ sự mệt mỏi, kiệt sức'),
    ('019a6500-0000-7fbc-89eb-7e0dab8def0b', 'style_d8466da19595ec63f7e8e6c0', 'Điệp từ cảm xúc'),
    ('019a6500-0000-7b83-bf07-c9fbf2825657', 'style_edcd91471c5f1bd5a6643a12', 'Điệp từ nhấn thời khắc'),
    ('019a6500-0000-77af-829a-6a6c63578091', 'style_acd0e9a019ede99894060b25', 'Điệp động từ bộc lộ cảm xúc'),
    ('019a6500-0000-78ea-bb6c-501766018d34', 'style_fccb789957e1120d72ce9931', 'Điệp động từ mạnh & tượng thanh va chạm'),
    ('019a6500-0000-7907-9732-b8669ea97868', 'style_435b4da01ed771236098b396', 'Đoạn miêu tả dài'),
    ('019a6500-0000-78fa-b3ca-bc528c27b6e4', 'style_cc0d32222467a5da5040b325', 'Đảo ngữ (đưa tân ngữ/trạng ngữ lên trước)'),
    ('019a6500-0000-7086-b5c4-8ec5a66a5796', 'style_3997d9e7af313624844c61f8', 'Đảo ngữ nhấn khoảnh khắc tiếp xúc'),
    ('019a6500-0000-747f-a30b-0abb4d3f0639', 'style_b5f8c6d64953b08adef53b84', 'Đảo ngữ nhấn lối thoát'),
    ('019a6500-0000-72c8-b7bb-575a6ef7abcd', 'style_be0aaba83dad759073c8a09f', 'Đối thoại với người khác'),
    ('019a6500-0000-7b88-b841-95c4a707a1a6', 'style_08560117641fab6f3c61eef3', 'Độc thoại nội tâm dài'),
    ('019a6500-0000-7f83-9718-5f5d4648ccc3', 'style_f92d9b7ee6494909972d3c2e', 'Ẩn dụ'),
    ('019a6500-0000-7ecf-a4fe-02112539db20', 'style_6063902ecb19e543c4c96b6d', 'Ẩn dụ bí mật chôn vùi & điệp cấu trúc dò đường'),
    ('019a6500-0000-7c47-a753-5374e2711503', 'style_ef3ed0b8e92af642e4451578', 'Ẩn dụ bóng tối & lời nói bóng gió'),
    ('019a6500-0000-7a9f-90f1-d696fbfac49d', 'style_33ee93b7701b6534d8cc1a2f', 'Ẩn dụ bức màn được vén lên'),
    ('019a6500-0000-70d6-b0e2-eb1733b98ec2', 'style_c5f3bd53c5084c027f7e9364', 'Ẩn dụ chữa lành & điệp nhịp êm dịu'),
    ('019a6500-0000-7697-b08f-9da2c0641fc8', 'style_eedc8134eed237acd76f78d8', 'Ẩn dụ cát chảy ngược'),
    ('019a6500-0000-768d-b2d1-72a394e4f1a0', 'style_a6c0e388748b1cd23203b0bb', 'Ẩn dụ dòng sông thời gian'),
    ('019a6500-0000-7aea-b194-64e85b72a37f', 'style_13acdfdeb5b971f8b4f13749', 'Ẩn dụ gắn kết & điệp cấu trúc trang trọng cân xứng'),
    ('019a6500-0000-7f33-b614-c4cd7b4a2ab2', 'style_61a5de7c398fc99b91227869', 'Ẩn dụ hoa mỹ'),
    ('019a6500-0000-7d5d-b5bc-3dad19c84878', 'style_30d4feb6d3b7c37f7db5b0dc', 'Ẩn dụ lửa / sóng'),
    ('019a6500-0000-7908-8786-125d5c51764c', 'style_bf5deb2763154c3c8c16dc9e', 'Ẩn dụ mạnh (ánh sáng/bóng tối, sông/núi)'),
    ('019a6500-0000-7046-a16d-1c0b36f0aaf0', 'style_d92e1622075499af1f65fa86', 'Ẩn dụ mất mát & điệp từ u ám'),
    ('019a6500-0000-732e-8684-2fc7e00f2bbe', 'style_7f6cb484bc1c8c0cfce39a40', 'Ẩn dụ rễ bị nhổ & điệp cấu trúc ngoảnh lại'),
    ('019a6500-0000-7eaa-bf3d-52795645eb87', 'style_ce6cdce4cdf7917b06e96697', 'Ẩn dụ triết lý'),
    ('019a6500-0000-760c-9fa4-07b788c9e9b5', 'style_146f4fad51b048ba4567a9a4', 'Ẩn dụ trữ tình'),
    ('019a6500-0000-728a-af43-335a353fb31c', 'style_a7809d59b8f7b0bca0f3f812', 'Ẩn dụ tình cảm & câu chấm lửng do dự'),
    ('019a6500-0000-73e7-86ca-be67c9017053', 'style_5880fe30d3cde357413f1a57', 'Ẩn dụ vết thương'),
    ('019a6500-0000-7724-a4fd-70c9db8e360a', 'style_b7e9af48a1723b6078746768', 'Ẩn dụ về cờ vua hoặc bàn cờ'),
    ('019a6500-0000-706e-96f9-e89d26a07c6b', 'style_c363406f8e0728edfe299ceb', 'Ẩn dụ về trọng lượng (gánh nặng) & câu hỏi tu từ đấu tranh'),
    ('019a6500-0000-7f3d-9745-d9ba40bf52f3', 'style_c56d55ecafaace15570d2f6f', 'Ẩn dụ ánh sáng / tự do & điệp cấu trúc cân xứng');

INSERT OR IGNORE INTO style_presets (id, code, category, tone_pace, tone_emotion, is_system, visual, auditory, tactile, olfactory, gustatory, language_abstract, syntax_sentence, syntax_variation, syntax_structure) VALUES
    ('019a6500-0000-7649-9191-8d23bb0272ef', 'action/assassination', 'action', 'VERY_LOW', 'căng thẳng nghẹt thở', '1', 'HIGH', 'LOW', 'HIGH', 'MEDIUM', 'NONE', 'MEDIUM', 'SHORT', 'HIGH', 'FRAGMENT'),
    ('019a6500-0000-73bf-8965-43db5c54bb13', 'action/battle', 'action', 'FAST', 'hỗn loạn, khẩn trương', '1', 'HIGH', 'HIGH', 'MEDIUM', 'MEDIUM', 'LOW', 'LOW', 'SHORT', 'HIGH', 'COMPOUND'),
    ('019a6500-0000-778d-abea-bc0c279898e7', 'action/chase', 'action', 'VERY_FAST', 'gấp gáp, khẩn cấp', '1', 'HIGH', 'HIGH', 'MEDIUM', 'LOW', 'NONE', 'LOW', 'VERY_SHORT', 'MEDIUM', 'SIMPLE'),
    ('019a6500-0000-7ba6-9823-fbd6d26f435c', 'action/combat', 'action', 'VERY_FAST', 'căng thẳng', '1', 'HIGH', 'HIGH', 'HIGH', 'LOW', 'NONE', 'LOW', 'SHORT', 'HIGH', 'SIMPLE'),
    ('019a6500-0000-7120-a6d1-779128366e02', 'action/duel', 'action', 'LOW', 'căng thẳng âm ỉ', '1', 'HIGH', 'MEDIUM', 'HIGH', 'LOW', 'NONE', 'MEDIUM', 'SHORT', 'HIGH', 'MIXED'),
    ('019a6500-0000-7305-ad33-f264f3fb90a1', 'action/escape', 'action', 'VERY_FAST', 'hoảng loạn, khẩn cấp', '1', 'HIGH', 'HIGH', 'MEDIUM', 'LOW', 'NONE', 'LOW', 'SHORT', 'HIGH', 'SIMPLE'),
    ('019a6500-0000-74f6-bd80-6fb76004f436', 'action/execution', 'action', 'VERY_LOW', 'lạnh lùng, dứt khoát', '1', 'HIGH', 'LOW', 'HIGH', 'LOW', 'NONE', 'LOW', 'VERY_SHORT', 'LOW', 'FRAGMENT'),
    ('019a6500-0000-7be9-a265-fd4d937cdf70', 'action/infiltration', 'action', 'LOW', 'thận trọng, căng thẳng ngầm', '1', 'HIGH', 'MEDIUM', 'MEDIUM', 'LOW', 'NONE', 'MEDIUM', 'MEDIUM', 'HIGH', 'MIXED'),
    ('019a6500-0000-76df-89bd-0a3cec6bf787', 'action/showdown', 'action', 'VERY_LOW', 'cực kỳ căng thẳng, xúc cảm dâng trào', '1', 'HIGH', 'MEDIUM', 'HIGH', 'LOW', 'NONE', 'HIGH', 'LONG', 'HIGH', 'COMPLEX'),
    ('019a6500-0000-7071-b143-4558d813cb0b', 'action/survival', 'action', 'LOW', 'khắc khoải, tuyệt vọng nhẹ', '1', 'HIGH', 'MEDIUM', 'HIGH', 'HIGH', 'MEDIUM', 'LOW', 'MEDIUM', 'MEDIUM', 'MIXED'),
    ('019a6500-0000-757d-9a4e-3784abbe108a', 'action/training', 'action', 'LOW', 'tập trung, quyết tâm', '1', 'MEDIUM', 'MEDIUM', 'HIGH', 'LOW', 'NONE', 'MEDIUM', 'MEDIUM', 'MEDIUM', 'MIXED'),
    ('019a6500-0000-7432-9d99-9a3a3bf6d095', 'action/warfare', 'action', 'LOW', 'lạnh lùng, tính toán', '1', 'HIGH', 'MEDIUM', 'LOW', 'LOW', 'NONE', 'HIGH', 'LONG', 'LOW', 'COMPLEX'),
    ('019a6500-0000-72a2-92cf-2095bad1f02e', 'exploration_discovery/first_contact', 'exploration_discovery', 'LOW', 'kinh ngạc, thận trọng, hồi hộp', '1', 'HIGH', 'MEDIUM', 'MEDIUM', 'MEDIUM', 'NONE', 'MEDIUM', 'MEDIUM', 'MEDIUM', 'COMPOUND'),
    ('019a6500-0000-7028-8c25-e1ddb07d06fc', 'exploration_discovery/map_unlocking', 'exploration_discovery', 'MODERATE', 'tò mò, phấn khích, kỳ vọng', '1', 'HIGH', 'MEDIUM', 'LOW', 'LOW', 'NONE', 'MEDIUM', 'SHORT', 'MEDIUM', 'COMPOUND'),
    ('019a6500-0000-72ff-940f-10282ee4595d', 'exploration_discovery/navigation_journey', 'exploration_discovery', 'MODERATE', 'kiên nhẫn, cảnh giác, quyết tâm', '1', 'HIGH', 'MEDIUM', 'MEDIUM', 'MEDIUM', 'LOW', 'LOW', 'MEDIUM', 'MEDIUM', 'COMPOUND'),
    ('019a6500-0000-750e-b05f-2308b8e27150', 'exploration_discovery/ruin_delving', 'exploration_discovery', 'LOW', 'bí ẩn, sợ hãi pha tò mò', '1', 'HIGH', 'HIGH', 'MEDIUM', 'MEDIUM', 'NONE', 'MEDIUM', 'MEDIUM', 'MEDIUM', 'COMPLEX'),
    ('019a6500-0000-7bc4-bc5f-6b653c1e6ab1', 'exploration_discovery/treasure_hunt', 'exploration_discovery', 'FAST', 'háo hức, căng thẳng, vui sướng', '1', 'HIGH', 'MEDIUM', 'MEDIUM', 'LOW', 'LOW', 'LOW', 'SHORT', 'HIGH', 'COMPOUND'),
    ('019a6500-0000-7ed3-a13e-b3d6813578f6', 'fantasy_scifi_mechanics/crafting_enhancement', 'fantasy_scifi_mechanics', 'LOW', 'tập trung, kiên nhẫn, thỏa mãn', '1', 'HIGH', 'MEDIUM', 'HIGH', 'MEDIUM', 'NONE', 'LOW', 'MEDIUM', 'MEDIUM', 'COMPOUND'),
    ('019a6500-0000-7226-969b-173b38137736', 'fantasy_scifi_mechanics/evolution_awakening', 'fantasy_scifi_mechanics', 'FAST', 'hào hứng, kinh ngạc, quyền lực', '1', 'HIGH', 'HIGH', 'HIGH', 'LOW', 'LOW', 'MEDIUM', 'SHORT', 'HIGH', 'COMPOUND'),
    ('019a6500-0000-7857-bbe4-22949902b0e1', 'fantasy_scifi_mechanics/hacking_decryption', 'fantasy_scifi_mechanics', 'FAST', 'tập trung cao độ, hồi hộp, chiến thắng', '1', 'HIGH', 'MEDIUM', 'LOW', 'NONE', 'NONE', 'MEDIUM', 'SHORT', 'HIGH', 'COMPOUND'),
    ('019a6500-0000-7a0d-87cc-c8f7d8826db8', 'fantasy_scifi_mechanics/summoning_ritual', 'fantasy_scifi_mechanics', 'LOW', 'thiêng liêng, sợ hãi, cuốn hút', '1', 'HIGH', 'HIGH', 'MEDIUM', 'MEDIUM', 'NONE', 'HIGH', 'LONG', 'MEDIUM', 'COMPLEX'),
    ('019a6500-0000-7cf5-aedc-d78a83018bc5', 'fantasy_scifi_mechanics/time_leap_loop', 'fantasy_scifi_mechanics', 'MODERATE', 'bối rối, quyết tâm, mệt mỏi, hy vọng', '1', 'MEDIUM', 'MEDIUM', 'LOW', 'LOW', 'NONE', 'HIGH', 'MEDIUM', 'MEDIUM', 'MIXED'),
    ('019a6500-0000-780b-8831-d89b6b3e40fc', 'mystery/alibi_check', 'mystery', 'LOW', 'khách quan, căng thẳng thầm lặng', '1', 'HIGH', 'LOW', 'LOW', 'NONE', 'NONE', 'LOW', 'MEDIUM', 'LOW', 'COMPLEX'),
    ('019a6500-0000-7fba-ab1e-aa10ad020bc9', 'mystery/clue_hunting', 'mystery', 'LOW', 'tò mò, căng thẳng âm ỉ', '1', 'HIGH', 'MEDIUM', 'MEDIUM', 'LOW', 'NONE', 'LOW', 'MEDIUM', 'LOW', 'COMPLEX'),
    ('019a6500-0000-733d-b3f0-5ec9336c80e6', 'mystery/deduction', 'mystery', 'LOW', 'tập trung, lạnh lùng', '1', 'LOW', 'LOW', 'NONE', 'NONE', 'NONE', 'HIGH', 'LONG', 'LOW', 'COMPLEX'),
    ('019a6500-0000-71cb-9ba7-6be304142484', 'mystery/disguise_investigation', 'mystery', 'MODERATE', 'căng thẳng ngấm ngầm, hồi hộp', '1', 'HIGH', 'MEDIUM', 'MEDIUM', 'LOW', 'NONE', 'LOW', 'MEDIUM', 'HIGH', 'COMPOUND'),
    ('019a6500-0000-7c1e-bbd5-583db9dfa7fd', 'mystery/interrogation', 'mystery', 'FAST', 'căng thẳng, áp lực', '1', 'MEDIUM', 'HIGH', 'LOW', 'NONE', 'NONE', 'MEDIUM', 'SHORT', 'HIGH', 'COMPOUND'),
    ('019a6500-0000-7b95-8330-e53c472be4a4', 'mystery/revelation', 'mystery', 'VERY_FAST', 'sốc, phẫn nộ, kinh hoàng', '1', 'HIGH', 'HIGH', 'MEDIUM', 'LOW', 'NONE', 'MEDIUM', 'SHORT', 'HIGH', 'COMPOUND'),
    ('019a6500-0000-7dd7-9636-097f120cb875', 'political_social/conspiracy_meeting', 'political_social', 'LOW', 'nguy hiểm, hoài nghi, mưu mô', '1', 'MEDIUM', 'HIGH', 'LOW', 'LOW', 'NONE', 'HIGH', 'MEDIUM', 'MEDIUM', 'MIXED'),
    ('019a6500-0000-75c2-be24-3bce2db1c90b', 'political_social/coronation_succession', 'political_social', 'LOW', 'uy nghiêm, căng thẳng ngầm, long trọng', '1', 'HIGH', 'HIGH', 'MEDIUM', 'MEDIUM', 'LOW', 'MEDIUM', 'LONG', 'LOW', 'COMPLEX'),
    ('019a6500-0000-7a33-a5a1-0f6b963f4787', 'political_social/exile_expulsion', 'political_social', 'LOW', 'tủi nhục, phẫn uất, cô độc, mất gốc', '1', 'HIGH', 'MEDIUM', 'MEDIUM', 'LOW', 'LOW', 'HIGH', 'MEDIUM', 'MEDIUM', 'MIXED'),
    ('019a6500-0000-739f-bd64-494f336e80cd', 'political_social/negotiation_treaty', 'political_social', 'LOW', 'căng thẳng, khôn khéo, thận trọng', '1', 'MEDIUM', 'HIGH', 'LOW', 'NONE', 'NONE', 'HIGH', 'MEDIUM', 'LOW', 'COMPLEX'),
    ('019a6500-0000-79f6-9471-7ab37e5da57b', 'political_social/public_speech', 'political_social', 'MODERATE', 'đầy cảm hứng, thuyết phục, oai hùng', '1', 'MEDIUM', 'HIGH', 'LOW', 'LOW', 'NONE', 'MEDIUM', 'SHORT', 'HIGH', 'COMPOUND'),
    ('019a6500-0000-7258-a9d5-09fc77154fbc', 'political_social/trial_court', 'political_social', 'MODERATE', 'nghiêm trang, hồi hộp, chờ phán quyết', '1', 'MEDIUM', 'HIGH', 'LOW', 'NONE', 'NONE', 'HIGH', 'MEDIUM', 'MEDIUM', 'COMPLEX'),
    ('019a6500-0000-74eb-be4a-4814423ecf6c', 'psychological_drama/betrayal', 'psychological_drama', 'FAST', 'sốc, đau đớn tột cùng, phẫn uất, hoang mang', '1', 'HIGH', 'HIGH', 'MEDIUM', 'LOW', 'LOW', 'MEDIUM', 'SHORT', 'HIGH', 'COMPOUND'),
    ('019a6500-0000-7284-888c-0d323eb04610', 'psychological_drama/identity_crisis', 'psychological_drama', 'LOW', 'mơ hồ, lo lắng, mất phương hướng', '1', 'MEDIUM', 'LOW', 'LOW', 'NONE', 'NONE', 'HIGH', 'LONG', 'HIGH', 'COMPLEX'),
    ('019a6500-0000-7680-a395-5f1e6328b5cc', 'psychological_drama/inner_monologue', 'psychological_drama', 'LOW', 'dằn vặt, đấu tranh, cô độc', '1', 'LOW', 'LOW', 'LOW', 'NONE', 'NONE', 'HIGH', 'LONG', 'MEDIUM', 'COMPLEX'),
    ('019a6500-0000-7826-93be-ba880b20a902', 'psychological_drama/nightmare_flashback', 'psychological_drama', 'VERY_FAST', 'hoảng loạn, kinh hoàng, bất lực', '1', 'HIGH', 'HIGH', 'HIGH', 'MEDIUM', 'LOW', 'MEDIUM', 'SHORT', 'HIGH', 'SIMPLE'),
    ('019a6500-0000-75b1-bb22-afb22c584a64', 'psychological_drama/sacrifice', 'psychological_drama', 'LOW', 'bi tráng, tĩnh lặng, bao dung, đau nhưng thanh thản', '1', 'HIGH', 'HIGH', 'HIGH', 'LOW', 'NONE', 'HIGH', 'LONG', 'LOW', 'COMPLEX'),
    ('019a6500-0000-7f39-b49d-8e438c9bb364', 'romance_relationship/confession', 'romance_relationship', 'LOW', 'mong manh, dũng cảm, hồi hộp', '1', 'HIGH', 'HIGH', 'MEDIUM', 'LOW', 'NONE', 'MEDIUM', 'SHORT', 'HIGH', 'COMPOUND'),
    ('019a6500-0000-7f82-a217-bda9aed38239', 'romance_relationship/date', 'romance_relationship', 'LOW', 'ấm áp, vui vẻ, thoải mái', '1', 'HIGH', 'MEDIUM', 'MEDIUM', 'MEDIUM', 'LOW', 'LOW', 'MEDIUM', 'LOW', 'COMPOUND'),
    ('019a6500-0000-7b93-8fa0-2accfcfdd6d9', 'romance_relationship/heartbreak', 'romance_relationship', 'LOW', 'đau buồn, tuyệt vọng, lặng lẽ', '1', 'HIGH', 'MEDIUM', 'MEDIUM', 'LOW', 'NONE', 'HIGH', 'LONG', 'LOW', 'COMPLEX'),
    ('019a6500-0000-7988-bc9c-c786b2a97e2a', 'romance_relationship/jealousy_confrontation', 'romance_relationship', 'FAST', 'nóng giận, tủi thân, bực bội', '1', 'HIGH', 'HIGH', 'MEDIUM', 'LOW', 'NONE', 'LOW', 'SHORT', 'HIGH', 'SIMPLE'),
    ('019a6500-0000-74a1-8a06-716442cbe3f6', 'romance_relationship/reunion', 'romance_relationship', 'LOW', 'xúc động, nghẹn ngào, mừng mừng tủi tủi', '1', 'HIGH', 'HIGH', 'HIGH', 'LOW', 'NONE', 'MEDIUM', 'SHORT', 'HIGH', 'COMPOUND'),
    ('019a6500-0000-7136-b533-7749359250e5', 'romance_relationship/wedding_ceremony', 'romance_relationship', 'LOW', 'hạnh phúc, thiêng liêng, xúc động', '1', 'HIGH', 'HIGH', 'MEDIUM', 'MEDIUM', 'LOW', 'MEDIUM', 'LONG', 'LOW', 'COMPLEX'),
    ('019a6500-0000-7013-a5c4-19de49103960', 'slice_of_life_comedy/daily_routine', 'slice_of_life_comedy', 'LOW', 'ấm cúng, thoải mái, buồn cười vì thói quen', '1', 'HIGH', 'MEDIUM', 'MEDIUM', 'MEDIUM', 'LOW', 'LOW', 'MEDIUM', 'LOW', 'COMPOUND'),
    ('019a6500-0000-710b-9e7f-a3f076a47db8', 'slice_of_life_comedy/feast_gathering', 'slice_of_life_comedy', 'MODERATE', 'hạnh phúc, ồn ào, đầm ấm', '1', 'HIGH', 'HIGH', 'MEDIUM', 'HIGH', 'HIGH', 'LOW', 'SHORT', 'MEDIUM', 'COMPOUND'),
    ('019a6500-0000-7279-a44e-11fa1d21e02a', 'slice_of_life_comedy/festival', 'slice_of_life_comedy', 'MODERATE', 'phấn khích, vui vẻ, náo nhiệt', '1', 'HIGH', 'HIGH', 'MEDIUM', 'MEDIUM', 'MEDIUM', 'LOW', 'SHORT', 'HIGH', 'COMPOUND'),
    ('019a6500-0000-75ef-b0d5-583af2593e02', 'slice_of_life_comedy/prank_misunderstanding', 'slice_of_life_comedy', 'FAST', 'bực bội nhẹ, buồn cười, hốt hoảng oái oăm', '1', 'HIGH', 'HIGH', 'MEDIUM', 'LOW', 'NONE', 'LOW', 'SHORT', 'HIGH', 'COMPOUND'),
    ('019a6500-0000-7ccf-aae9-5dbc419436e9', 'slice_of_life_comedy/shopping_preparation', 'slice_of_life_comedy', 'MODERATE', 'hào hứng, thoải mái, tỉ mỉ', '1', 'HIGH', 'MEDIUM', 'MEDIUM', 'MEDIUM', 'LOW', 'LOW', 'SHORT', 'MEDIUM', 'COMPOUND'),
    ('019a6500-0000-7a20-96a3-46f0b10badb9', 'transition/recovery_rest', 'transition', 'LOW', 'thư thái, hồi phục, bình yên', '1', 'MEDIUM', 'LOW', 'MEDIUM', 'MEDIUM', 'LOW', 'MEDIUM', 'MEDIUM', 'LOW', 'COMPLEX'),
    ('019a6500-0000-7c26-b666-49b954ef73b5', 'transition/time_skip', 'transition', 'FAST', 'hoài niệm, mới mẻ, trăn trở về thay đổi', '1', 'MEDIUM', 'LOW', 'LOW', 'LOW', 'NONE', 'MEDIUM', 'SHORT', 'HIGH', 'COMPOUND'),
    ('019a6500-0000-78e7-9c4a-57f623bac127', 'transition/travel_montage', 'transition', 'FAST', 'phiêu lưu, phấn khích, hân hoan', '1', 'HIGH', 'MEDIUM', 'MEDIUM', 'MEDIUM', 'LOW', 'LOW', 'SHORT', 'HIGH', 'COMPOUND'),
    ('019a6500-0000-79ee-8a9a-5d6196bfb8a4', 'transition/waiting_anticipation', 'transition', 'LOW', 'hồi hộp, sốt ruột, lo lắng, hy vọng', '1', 'MEDIUM', 'MEDIUM', 'LOW', 'LOW', 'NONE', 'MEDIUM', 'MEDIUM', 'MEDIUM', 'MIXED');

INSERT OR IGNORE INTO style_preset_rhetorical_devices (id, preset_id, rhetorical_device_id, usage_type, custom_instruction) VALUES
    ('019a6500-0000-7f01-b8dd-5e713f40e6e1', '019a6500-0000-7649-9191-8d23bb0272ef', '019a6500-0000-7f83-9718-5f5d4648ccc3', 'primary', NULL),
    ('019a6500-0000-7362-9f0a-55ab19aeb165', '019a6500-0000-7649-9191-8d23bb0272ef', '019a6500-0000-70fe-8cec-d5668478fa16', 'secondary', NULL),
    ('019a6500-0000-7f25-8f85-1d4277b0ac3a', '019a6500-0000-7649-9191-8d23bb0272ef', '019a6500-0000-7086-b5c4-8ec5a66a5796', 'secondary', NULL),
    ('019a6500-0000-7ac5-b0b2-6697c264ed47', '019a6500-0000-7649-9191-8d23bb0272ef', '019a6500-0000-77fc-b583-31e15c3f8de2', 'avoid', NULL),
    ('019a6500-0000-73d9-b111-2cad751a44d4', '019a6500-0000-7649-9191-8d23bb0272ef', '019a6500-0000-7614-bd96-aadcf55d8aaf', 'avoid', NULL),
    ('019a6500-0000-74a6-8369-0b057af18515', '019a6500-0000-73bf-8965-43db5c54bb13', '019a6500-0000-70c4-a6ec-4cbd7af2a03e', 'primary', NULL),
    ('019a6500-0000-7f1d-bf5d-cf84e21fd62c', '019a6500-0000-73bf-8965-43db5c54bb13', '019a6500-0000-7f61-b2aa-acc298af9b34', 'secondary', NULL),
    ('019a6500-0000-78f0-acbe-22746ceb60bc', '019a6500-0000-73bf-8965-43db5c54bb13', '019a6500-0000-739e-ac77-8a8bbf40e77e', 'secondary', NULL),
    ('019a6500-0000-7419-80cb-1e9230a8b1f4', '019a6500-0000-73bf-8965-43db5c54bb13', '019a6500-0000-78d8-a97d-1beb0bb22289', 'avoid', NULL),
    ('019a6500-0000-7730-af1a-d3baf642ada5', '019a6500-0000-73bf-8965-43db5c54bb13', '019a6500-0000-7e2f-b6e7-9db2793291a6', 'avoid', NULL),
    ('019a6500-0000-7cb9-bb4f-0160cab510ce', '019a6500-0000-778d-abea-bc0c279898e7', '019a6500-0000-7817-a97e-c9b004979770', 'primary', NULL),
    ('019a6500-0000-70e6-a7cb-601229aa9615', '019a6500-0000-778d-abea-bc0c279898e7', '019a6500-0000-7a72-ac21-8b8d9a91da38', 'secondary', NULL),
    ('019a6500-0000-7583-a84e-ed49e70947bb', '019a6500-0000-778d-abea-bc0c279898e7', '019a6500-0000-7baf-bc4d-240cd0896f11', 'secondary', NULL),
    ('019a6500-0000-7255-b489-7150af58a586', '019a6500-0000-778d-abea-bc0c279898e7', '019a6500-0000-7ea0-be76-4da7b8eb3e26', 'avoid', NULL),
    ('019a6500-0000-789e-8e89-7c1fb33855c0', '019a6500-0000-778d-abea-bc0c279898e7', '019a6500-0000-760c-9fa4-07b788c9e9b5', 'avoid', NULL),
    ('019a6500-0000-73d4-a690-8b0023970f32', '019a6500-0000-7ba6-9823-fbd6d26f435c', '019a6500-0000-78ea-bb6c-501766018d34', 'primary', NULL),
    ('019a6500-0000-7cd1-9eb7-e1ede3c56a00', '019a6500-0000-7ba6-9823-fbd6d26f435c', '019a6500-0000-7801-ae1b-9635fa0a0570', 'secondary', NULL),
    ('019a6500-0000-7e36-9303-1ff3eba436da', '019a6500-0000-7ba6-9823-fbd6d26f435c', '019a6500-0000-756b-a916-3451692b9c56', 'secondary', NULL),
    ('019a6500-0000-7e96-b4d0-7b4c68c4f0b2', '019a6500-0000-7ba6-9823-fbd6d26f435c', '019a6500-0000-760c-9fa4-07b788c9e9b5', 'avoid', NULL),
    ('019a6500-0000-7038-b4a9-ea1742884933', '019a6500-0000-7ba6-9823-fbd6d26f435c', '019a6500-0000-7ea0-be76-4da7b8eb3e26', 'avoid', NULL),
    ('019a6500-0000-7379-a1f6-e94f6ef5eefd', '019a6500-0000-7120-a6d1-779128366e02', '019a6500-0000-7aea-a555-e1fb819b7433', 'primary', NULL),
    ('019a6500-0000-7bf5-b540-e53917f587c4', '019a6500-0000-7120-a6d1-779128366e02', '019a6500-0000-78f7-9fff-482f1d039cfe', 'secondary', NULL),
    ('019a6500-0000-76d3-a238-7c1ed3c1974d', '019a6500-0000-7120-a6d1-779128366e02', '019a6500-0000-7a68-84ca-ebb25cce8bf3', 'secondary', NULL),
    ('019a6500-0000-7e91-ba5c-731788147be3', '019a6500-0000-7120-a6d1-779128366e02', '019a6500-0000-7446-ab65-5c315d456cf9', 'avoid', NULL),
    ('019a6500-0000-747b-9ce9-42e62479efa7', '019a6500-0000-7120-a6d1-779128366e02', '019a6500-0000-7fd7-a705-19bd85dc15fe', 'avoid', NULL),
    ('019a6500-0000-7ca3-ac6d-5e45c332b0a9', '019a6500-0000-7305-ad33-f264f3fb90a1', '019a6500-0000-762c-a457-94949052a544', 'primary', NULL),
    ('019a6500-0000-7206-a3dd-6fb34ea10be1', '019a6500-0000-7305-ad33-f264f3fb90a1', '019a6500-0000-7b38-902f-f90232096656', 'secondary', NULL),
    ('019a6500-0000-7c34-b6e2-ea42b241be2a', '019a6500-0000-7305-ad33-f264f3fb90a1', '019a6500-0000-747f-a30b-0abb4d3f0639', 'secondary', NULL),
    ('019a6500-0000-718a-ac18-423bcca9073d', '019a6500-0000-7305-ad33-f264f3fb90a1', '019a6500-0000-7653-a9f8-db4c74855ed5', 'avoid', NULL),
    ('019a6500-0000-763e-93d8-28e3fca2650b', '019a6500-0000-7305-ad33-f264f3fb90a1', '019a6500-0000-7eaa-bf3d-52795645eb87', 'avoid', NULL),
    ('019a6500-0000-70dd-9677-fb7a7abb0c05', '019a6500-0000-74f6-bd80-6fb76004f436', '019a6500-0000-76eb-a55b-02bd96a748dd', 'primary', NULL),
    ('019a6500-0000-75c7-a9af-661bce8a10ae', '019a6500-0000-74f6-bd80-6fb76004f436', '019a6500-0000-78fa-b3ca-bc528c27b6e4', 'secondary', NULL),
    ('019a6500-0000-7561-a7d1-1f086dbb46b6', '019a6500-0000-74f6-bd80-6fb76004f436', '019a6500-0000-7e8e-99ca-6e5bf7fac5c0', 'secondary', NULL),
    ('019a6500-0000-7c82-a981-4763b2a0f903', '019a6500-0000-74f6-bd80-6fb76004f436', '019a6500-0000-7f33-b614-c4cd7b4a2ab2', 'avoid', NULL),
    ('019a6500-0000-7d5b-97fa-164fdafb2776', '019a6500-0000-74f6-bd80-6fb76004f436', '019a6500-0000-76c8-9e89-5a7faaed7d8d', 'avoid', NULL),
    ('019a6500-0000-71c8-b668-3851b8cb5ac7', '019a6500-0000-74f6-bd80-6fb76004f436', '019a6500-0000-7616-876c-b27bd9fa9425', 'avoid', NULL),
    ('019a6500-0000-79f1-9e9c-08683f046c0a', '019a6500-0000-7be9-a265-fd4d937cdf70', '019a6500-0000-7cb2-8028-aba4eb88cc60', 'primary', NULL),
    ('019a6500-0000-72b9-b083-4430ee4da6b7', '019a6500-0000-7be9-a265-fd4d937cdf70', '019a6500-0000-71e8-8c0c-8cf537f03cca', 'secondary', NULL),
    ('019a6500-0000-784d-b039-6724f4320a82', '019a6500-0000-7be9-a265-fd4d937cdf70', '019a6500-0000-7c63-8500-0fdd86243288', 'secondary', NULL),
    ('019a6500-0000-7188-803c-93cf7908a992', '019a6500-0000-7be9-a265-fd4d937cdf70', '019a6500-0000-7266-b506-1ba0dec19e57', 'avoid', NULL),
    ('019a6500-0000-7d4b-9e86-89717c7db9e2', '019a6500-0000-7be9-a265-fd4d937cdf70', '019a6500-0000-7446-ab65-5c315d456cf9', 'avoid', NULL),
    ('019a6500-0000-764c-a781-053e955edd80', '019a6500-0000-76df-89bd-0a3cec6bf787', '019a6500-0000-7eab-bb3f-3cb033412f55', 'primary', NULL),
    ('019a6500-0000-78c7-8915-3707c4d1905a', '019a6500-0000-76df-89bd-0a3cec6bf787', '019a6500-0000-7f2e-a428-fc87be692303', 'secondary', NULL),
    ('019a6500-0000-7626-8385-5f44b88e3d34', '019a6500-0000-76df-89bd-0a3cec6bf787', '019a6500-0000-7908-8786-125d5c51764c', 'secondary', NULL),
    ('019a6500-0000-701f-9167-b35991c8b9c0', '019a6500-0000-76df-89bd-0a3cec6bf787', '019a6500-0000-7b83-bf07-c9fbf2825657', 'secondary', NULL),
    ('019a6500-0000-7596-8dd6-670539dab815', '019a6500-0000-76df-89bd-0a3cec6bf787', '019a6500-0000-7ebb-8ebe-884d9eface7e', 'avoid', NULL),
    ('019a6500-0000-7d95-9b1e-d9f3c7940268', '019a6500-0000-76df-89bd-0a3cec6bf787', '019a6500-0000-76ff-bcb3-40fcd39d3627', 'avoid', NULL),
    ('019a6500-0000-79b2-93c6-45df0fa80e8a', '019a6500-0000-7071-b143-4558d813cb0b', '019a6500-0000-7bcf-a9e9-625a3b6edcd1', 'primary', NULL),
    ('019a6500-0000-7ade-a86a-c80c152545d7', '019a6500-0000-7071-b143-4558d813cb0b', '019a6500-0000-7ffb-bc8e-24fc5386c19c', 'secondary', NULL),
    ('019a6500-0000-7e1b-b142-8d580f5ab72a', '019a6500-0000-7071-b143-4558d813cb0b', '019a6500-0000-717f-a624-d4c7f3aee3eb', 'secondary', NULL),
    ('019a6500-0000-7dac-9707-d795ddfd0789', '019a6500-0000-7071-b143-4558d813cb0b', '019a6500-0000-7635-88b4-c34e3d379270', 'avoid', NULL),
    ('019a6500-0000-7eb5-a883-a20dfa6c1a1b', '019a6500-0000-7071-b143-4558d813cb0b', '019a6500-0000-7a67-9d23-d8a2fa5ca5f3', 'avoid', NULL),
    ('019a6500-0000-7c8a-b1af-0958659e63ff', '019a6500-0000-757d-9a4e-3784abbe108a', '019a6500-0000-796b-a7e6-dc3a8b21f237', 'primary', NULL),
    ('019a6500-0000-7601-9bd8-c39bdc05faac', '019a6500-0000-757d-9a4e-3784abbe108a', '019a6500-0000-7012-946c-98b79419c78f', 'secondary', NULL),
    ('019a6500-0000-7134-8535-7cf80d30f631', '019a6500-0000-757d-9a4e-3784abbe108a', '019a6500-0000-7490-b42c-ca18bdbd29cc', 'secondary', NULL),
    ('019a6500-0000-731f-ba29-44a8acbfe7b9', '019a6500-0000-757d-9a4e-3784abbe108a', '019a6500-0000-7f83-9718-5f5d4648ccc3', 'avoid', NULL),
    ('019a6500-0000-7c12-a7b0-7c866a2deb44', '019a6500-0000-757d-9a4e-3784abbe108a', '019a6500-0000-7fbc-89eb-7e0dab8def0b', 'avoid', NULL),
    ('019a6500-0000-71ce-b27f-4e9f7940b58f', '019a6500-0000-757d-9a4e-3784abbe108a', '019a6500-0000-7616-876c-b27bd9fa9425', 'avoid', NULL),
    ('019a6500-0000-78b8-bd7d-1db7e473f2f8', '019a6500-0000-7432-9d99-9a3a3bf6d095', '019a6500-0000-79b8-9063-0360456599ee', 'primary', NULL),
    ('019a6500-0000-73f2-9090-3663ab01912b', '019a6500-0000-7432-9d99-9a3a3bf6d095', '019a6500-0000-7ba9-8bfd-7da9e1603fd9', 'secondary', NULL),
    ('019a6500-0000-70cf-aa5f-25c7fae149d5', '019a6500-0000-7432-9d99-9a3a3bf6d095', '019a6500-0000-7724-a4fd-70c9db8e360a', 'secondary', NULL),
    ('019a6500-0000-735e-8883-cce6d2034be9', '019a6500-0000-7432-9d99-9a3a3bf6d095', '019a6500-0000-7d61-978a-a0f2f9ccdf1d', 'avoid', NULL),
    ('019a6500-0000-7c96-860a-0386aedb0abc', '019a6500-0000-7432-9d99-9a3a3bf6d095', '019a6500-0000-785f-8d1a-d0deea475375', 'avoid', NULL),
    ('019a6500-0000-7ec4-8b25-f10883751658', '019a6500-0000-72a2-92cf-2095bad1f02e', '019a6500-0000-7bda-aee3-4a395fc553cc', 'primary', NULL),
    ('019a6500-0000-764b-be54-0a97876d9267', '019a6500-0000-72a2-92cf-2095bad1f02e', '019a6500-0000-7dc2-a55e-77b5297c31c0', 'secondary', NULL),
    ('019a6500-0000-7161-9683-d51f49290c80', '019a6500-0000-72a2-92cf-2095bad1f02e', '019a6500-0000-752e-8417-50ba8c10f3e9', 'secondary', NULL),
    ('019a6500-0000-79f7-814f-ae117043704a', '019a6500-0000-72a2-92cf-2095bad1f02e', '019a6500-0000-7e5e-826a-48afc36611c4', 'avoid', NULL),
    ('019a6500-0000-7377-922f-30bae9f04da5', '019a6500-0000-72a2-92cf-2095bad1f02e', '019a6500-0000-7ed7-ad1a-704c8e39e19e', 'avoid', NULL),
    ('019a6500-0000-7800-a9eb-3656cf88eaac', '019a6500-0000-7028-8c25-e1ddb07d06fc', '019a6500-0000-7477-be0f-45765040c99b', 'primary', NULL),
    ('019a6500-0000-7431-b5a2-a257ec1d6994', '019a6500-0000-7028-8c25-e1ddb07d06fc', '019a6500-0000-7a9f-90f1-d696fbfac49d', 'secondary', NULL),
    ('019a6500-0000-7303-9455-ce350bcabef6', '019a6500-0000-7028-8c25-e1ddb07d06fc', '019a6500-0000-70c1-ad10-f694eef3b333', 'secondary', NULL),
    ('019a6500-0000-7387-861f-39b5bc43b125', '019a6500-0000-7028-8c25-e1ddb07d06fc', '019a6500-0000-7932-b4de-f7ff7361994a', 'avoid', NULL),
    ('019a6500-0000-746d-83c6-a620faa5082e', '019a6500-0000-7028-8c25-e1ddb07d06fc', '019a6500-0000-79d1-979a-fe05d541579d', 'avoid', NULL),
    ('019a6500-0000-7e4a-b4df-afef46ce07ff', '019a6500-0000-72ff-940f-10282ee4595d', '019a6500-0000-7407-9a6c-561005a85ff9', 'primary', NULL),
    ('019a6500-0000-70c1-a37c-a67c95feee1c', '019a6500-0000-72ff-940f-10282ee4595d', '019a6500-0000-717d-80ad-bdff81a502ce', 'secondary', NULL),
    ('019a6500-0000-70fc-a5e0-cc39682aba13', '019a6500-0000-72ff-940f-10282ee4595d', '019a6500-0000-7072-acc4-e79f692a556d', 'secondary', NULL),
    ('019a6500-0000-778f-8b9d-0111c36611cc', '019a6500-0000-72ff-940f-10282ee4595d', '019a6500-0000-781a-b79e-8088aaf4e3f3', 'avoid', NULL),
    ('019a6500-0000-7fc4-83c7-60bbacbe9e8a', '019a6500-0000-72ff-940f-10282ee4595d', '019a6500-0000-73b7-8ce3-3bcfebe079f4', 'avoid', NULL),
    ('019a6500-0000-7deb-a513-e01d66f4035b', '019a6500-0000-750e-b05f-2308b8e27150', '019a6500-0000-7ecf-a4fe-02112539db20', 'primary', NULL),
    ('019a6500-0000-7775-ba17-d034f10bef45', '019a6500-0000-750e-b05f-2308b8e27150', '019a6500-0000-74cc-b180-e34db3d32996', 'secondary', NULL),
    ('019a6500-0000-7cc7-880c-298a3fc712b5', '019a6500-0000-750e-b05f-2308b8e27150', '019a6500-0000-7cef-99e1-e12cd76fb65e', 'secondary', NULL),
    ('019a6500-0000-78bf-8308-9f3805612726', '019a6500-0000-750e-b05f-2308b8e27150', '019a6500-0000-7ecb-8525-3f9475e02197', 'avoid', NULL),
    ('019a6500-0000-7f05-9cdd-9e7c0ecec1c2', '019a6500-0000-750e-b05f-2308b8e27150', '019a6500-0000-7446-ab65-5c315d456cf9', 'avoid', NULL),
    ('019a6500-0000-7b11-abee-15a0cffa9b60', '019a6500-0000-7bc4-bc5f-6b653c1e6ab1', '019a6500-0000-76b9-b225-0c7828a58e99', 'primary', NULL),
    ('019a6500-0000-7b89-a12e-24a784ebbb5a', '019a6500-0000-7bc4-bc5f-6b653c1e6ab1', '019a6500-0000-7d52-8aac-f29adccecd12', 'secondary', NULL),
    ('019a6500-0000-769d-b008-46fb12a199d6', '019a6500-0000-7bc4-bc5f-6b653c1e6ab1', '019a6500-0000-7048-8313-fe6e101763a7', 'secondary', NULL),
    ('019a6500-0000-7103-ac25-49ba8e0bc5e8', '019a6500-0000-7bc4-bc5f-6b653c1e6ab1', '019a6500-0000-795d-9167-8746858887fc', 'avoid', NULL),
    ('019a6500-0000-7f29-8e22-3d8ccb98d65f', '019a6500-0000-7bc4-bc5f-6b653c1e6ab1', '019a6500-0000-73b7-8ce3-3bcfebe079f4', 'avoid', NULL),
    ('019a6500-0000-738b-8f1f-9f1e86cda2f9', '019a6500-0000-7ed3-a13e-b3d6813578f6', '019a6500-0000-7fb3-9032-db7f95f2602e', 'primary', NULL),
    ('019a6500-0000-7a04-bd21-486bf5033e5c', '019a6500-0000-7ed3-a13e-b3d6813578f6', '019a6500-0000-7b3e-a378-c688047944a6', 'secondary', NULL),
    ('019a6500-0000-7bff-afa6-0caef7c09854', '019a6500-0000-7ed3-a13e-b3d6813578f6', '019a6500-0000-7f73-81b7-cf42ad2e4b95', 'secondary', NULL),
    ('019a6500-0000-7ef0-9e0d-9b66de3bba99', '019a6500-0000-7ed3-a13e-b3d6813578f6', '019a6500-0000-7383-a78a-9ffc0972bef9', 'avoid', NULL),
    ('019a6500-0000-7b0f-a9bc-6582aa6d6ba8', '019a6500-0000-7ed3-a13e-b3d6813578f6', '019a6500-0000-747d-a391-ab757ed1a948', 'avoid', NULL),
    ('019a6500-0000-7e4c-ad4a-bea14e20addd', '019a6500-0000-7226-969b-173b38137736', '019a6500-0000-7e8a-8bb8-c33928b2cf88', 'primary', NULL),
    ('019a6500-0000-7f89-af61-9e3c80c1f9dd', '019a6500-0000-7226-969b-173b38137736', '019a6500-0000-7d5d-b5bc-3dad19c84878', 'secondary', NULL),
    ('019a6500-0000-777e-8028-3e96d7123ae9', '019a6500-0000-7226-969b-173b38137736', '019a6500-0000-7ff9-8343-09607f3edc8d', 'secondary', NULL),
    ('019a6500-0000-7a87-8bed-9ea5a5ca48c7', '019a6500-0000-7226-969b-173b38137736', '019a6500-0000-7093-a8d8-10192b3dda77', 'avoid', NULL),
    ('019a6500-0000-7d59-a154-66add9d2d598', '019a6500-0000-7226-969b-173b38137736', '019a6500-0000-7381-9fb2-e6a796aa8c44', 'avoid', NULL),
    ('019a6500-0000-7852-a4c6-6bb969a57064', '019a6500-0000-7857-bbe4-22949902b0e1', '019a6500-0000-7c5e-9ebf-a61924c8631d', 'primary', NULL),
    ('019a6500-0000-74f5-a025-5466b9e14480', '019a6500-0000-7857-bbe4-22949902b0e1', '019a6500-0000-7264-95a7-1169d9fb67ae', 'secondary', NULL),
    ('019a6500-0000-7930-a1bd-ec0c68cc40fd', '019a6500-0000-7857-bbe4-22949902b0e1', '019a6500-0000-7355-a878-c6681553add7', 'secondary', NULL),
    ('019a6500-0000-79ec-b867-feabb55151ef', '019a6500-0000-7857-bbe4-22949902b0e1', '019a6500-0000-7e39-9fcc-6762a51560bb', 'avoid', NULL),
    ('019a6500-0000-7055-9225-15a550d47a2b', '019a6500-0000-7857-bbe4-22949902b0e1', '019a6500-0000-7fd7-a705-19bd85dc15fe', 'avoid', NULL),
    ('019a6500-0000-7027-a39e-d7a6d8d66bac', '019a6500-0000-7a0d-87cc-c8f7d8826db8', '019a6500-0000-7b43-a3a4-40cb5bcdbd65', 'primary', NULL),
    ('019a6500-0000-7df1-b448-32d8382bcbb8', '019a6500-0000-7a0d-87cc-c8f7d8826db8', '019a6500-0000-72aa-a894-cc6618eb0fc8', 'secondary', NULL),
    ('019a6500-0000-76fc-8e1a-e9a61fb47e8c', '019a6500-0000-7a0d-87cc-c8f7d8826db8', '019a6500-0000-767e-8215-4fbda2ebf0a5', 'secondary', NULL),
    ('019a6500-0000-7453-b96b-ee3135ed419b', '019a6500-0000-7a0d-87cc-c8f7d8826db8', '019a6500-0000-7c30-a504-026fbecda593', 'avoid', NULL),
    ('019a6500-0000-7b93-809b-452a5bc8e0b1', '019a6500-0000-7a0d-87cc-c8f7d8826db8', '019a6500-0000-781e-890c-26a2ab1e860a', 'avoid', NULL),
    ('019a6500-0000-788d-8d75-9512201bcd27', '019a6500-0000-7cf5-aedc-d78a83018bc5', '019a6500-0000-79bf-a94c-99712132f284', 'primary', NULL),
    ('019a6500-0000-731e-9145-0024fd3f3951', '019a6500-0000-7cf5-aedc-d78a83018bc5', '019a6500-0000-781e-a563-a124ad5088b8', 'secondary', NULL),
    ('019a6500-0000-71e2-9c5e-76014374a7d6', '019a6500-0000-7cf5-aedc-d78a83018bc5', '019a6500-0000-7697-b08f-9da2c0641fc8', 'secondary', NULL),
    ('019a6500-0000-7e78-b317-e0f283564e49', '019a6500-0000-7cf5-aedc-d78a83018bc5', '019a6500-0000-74ec-bf94-928e80cafa03', 'avoid', NULL),
    ('019a6500-0000-7087-8b46-e6208fe6c29f', '019a6500-0000-7cf5-aedc-d78a83018bc5', '019a6500-0000-7b4c-b4c4-42533de734d1', 'avoid', NULL),
    ('019a6500-0000-7aee-84af-59606a2b7770', '019a6500-0000-780b-8831-d89b6b3e40fc', '019a6500-0000-7de0-8a98-cb44b6b00fa1', 'primary', NULL),
    ('019a6500-0000-73f4-aa56-cdb0f8e7723d', '019a6500-0000-780b-8831-d89b6b3e40fc', '019a6500-0000-73ae-8da5-274da7ae0461', 'secondary', NULL),
    ('019a6500-0000-725c-8023-630fec019a10', '019a6500-0000-780b-8831-d89b6b3e40fc', '019a6500-0000-7ff0-9baf-11d6d4a3feb4', 'secondary', NULL),
    ('019a6500-0000-71df-9c1e-df3484ddc3cf', '019a6500-0000-780b-8831-d89b6b3e40fc', '019a6500-0000-7616-876c-b27bd9fa9425', 'avoid', NULL),
    ('019a6500-0000-7095-8f69-d62762bde1e4', '019a6500-0000-780b-8831-d89b6b3e40fc', '019a6500-0000-76cd-9161-d28eae5790b2', 'avoid', NULL),
    ('019a6500-0000-7ea2-9263-6befe4b1f9f0', '019a6500-0000-7fba-ab1e-aa10ad020bc9', '019a6500-0000-70d0-8e97-910219bdf08c', 'primary', NULL),
    ('019a6500-0000-729b-8838-041907e1f05e', '019a6500-0000-7fba-ab1e-aa10ad020bc9', '019a6500-0000-7f34-b05c-24a60eb8807f', 'secondary', NULL),
    ('019a6500-0000-7b69-9a89-f179bcbeba43', '019a6500-0000-7fba-ab1e-aa10ad020bc9', '019a6500-0000-7eb7-a61f-a342921a5972', 'secondary', NULL),
    ('019a6500-0000-7fcd-96b8-88a540a56f7e', '019a6500-0000-7fba-ab1e-aa10ad020bc9', '019a6500-0000-7ba1-bc4a-dc9970bb4782', 'avoid', NULL),
    ('019a6500-0000-7fbb-83d8-fba3b0956b44', '019a6500-0000-7fba-ab1e-aa10ad020bc9', '019a6500-0000-775c-a6b2-136efc2fa6cc', 'avoid', NULL),
    ('019a6500-0000-7d32-8fad-8a21e32ee88e', '019a6500-0000-733d-b3f0-5ec9336c80e6', '019a6500-0000-7a50-ac6c-1db4014c81ad', 'primary', NULL),
    ('019a6500-0000-7079-adf6-882b24255c16', '019a6500-0000-733d-b3f0-5ec9336c80e6', '019a6500-0000-711d-bc9a-90f122a339b8', 'secondary', NULL),
    ('019a6500-0000-7db2-af82-b3e1442ec786', '019a6500-0000-733d-b3f0-5ec9336c80e6', '019a6500-0000-707c-9d2f-f7d50ac00db8', 'secondary', NULL),
    ('019a6500-0000-7787-bef7-c761b2b2ef68', '019a6500-0000-733d-b3f0-5ec9336c80e6', '019a6500-0000-7614-bd96-aadcf55d8aaf', 'avoid', NULL),
    ('019a6500-0000-792d-a71d-2908833fbaae', '019a6500-0000-733d-b3f0-5ec9336c80e6', '019a6500-0000-7fd8-9745-b9fbc131d658', 'avoid', NULL),
    ('019a6500-0000-7b45-876c-bf48cd5da482', '019a6500-0000-71cb-9ba7-6be304142484', '019a6500-0000-703f-998c-bf1a68ed07d5', 'primary', NULL),
    ('019a6500-0000-7dcd-b176-e559ced70447', '019a6500-0000-71cb-9ba7-6be304142484', '019a6500-0000-788f-8a70-7b8ae6c72622', 'secondary', NULL),
    ('019a6500-0000-709a-839d-bd2642c165be', '019a6500-0000-71cb-9ba7-6be304142484', '019a6500-0000-7b1a-b429-784133dcba7f', 'secondary', NULL),
    ('019a6500-0000-7a24-87e9-cfaa695961a0', '019a6500-0000-71cb-9ba7-6be304142484', '019a6500-0000-7b88-b841-95c4a707a1a6', 'avoid', NULL),
    ('019a6500-0000-7fcf-87ea-aea0c834fc27', '019a6500-0000-71cb-9ba7-6be304142484', '019a6500-0000-7d43-8e6d-06b04e1ae196', 'avoid', NULL),
    ('019a6500-0000-797f-94d3-f7e067fb1f2c', '019a6500-0000-7c1e-bbd5-583db9dfa7fd', '019a6500-0000-7975-b0c8-fa67924f7a9e', 'primary', NULL),
    ('019a6500-0000-7d56-b73e-ae11173611a4', '019a6500-0000-7c1e-bbd5-583db9dfa7fd', '019a6500-0000-755b-a3b4-50b7cf07f5b9', 'secondary', NULL),
    ('019a6500-0000-72ae-9eb2-d49517b2d961', '019a6500-0000-7c1e-bbd5-583db9dfa7fd', '019a6500-0000-7f6e-956b-88e6ef323226', 'secondary', NULL),
    ('019a6500-0000-7160-b39d-03ace57111aa', '019a6500-0000-7c1e-bbd5-583db9dfa7fd', '019a6500-0000-7907-9732-b8669ea97868', 'avoid', NULL),
    ('019a6500-0000-7c4d-9a12-11409a45d592', '019a6500-0000-7c1e-bbd5-583db9dfa7fd', '019a6500-0000-7920-8c38-16fc7d7483a0', 'avoid', NULL),
    ('019a6500-0000-7ab5-8268-30269677079e', '019a6500-0000-7b95-8330-e53c472be4a4', '019a6500-0000-782b-a4ff-663b90f37366', 'primary', NULL),
    ('019a6500-0000-7909-9287-3147e7515021', '019a6500-0000-7b95-8330-e53c472be4a4', '019a6500-0000-7ab6-b59a-3c1c6adce03e', 'secondary', NULL),
    ('019a6500-0000-70b8-a264-a3ced937dfee', '019a6500-0000-7b95-8330-e53c472be4a4', '019a6500-0000-7179-9ccf-eadf87255938', 'secondary', NULL),
    ('019a6500-0000-742d-bbb4-f578cc7243ee', '019a6500-0000-7b95-8330-e53c472be4a4', '019a6500-0000-7bd8-9379-afcdc7cab0d1', 'avoid', NULL),
    ('019a6500-0000-7cf7-99de-5ceadb74fcc6', '019a6500-0000-7b95-8330-e53c472be4a4', '019a6500-0000-7789-b89d-3eafb6e2ef4e', 'avoid', NULL),
    ('019a6500-0000-7401-a8b0-6054f324c9f4', '019a6500-0000-7dd7-9636-097f120cb875', '019a6500-0000-7c47-a753-5374e2711503', 'primary', NULL),
    ('019a6500-0000-72e5-b84a-58758ee16e3a', '019a6500-0000-7dd7-9636-097f120cb875', '019a6500-0000-7cbf-81c9-4dedb4256f4d', 'secondary', NULL),
    ('019a6500-0000-7c31-8dcc-90ab1acee8b1', '019a6500-0000-7dd7-9636-097f120cb875', '019a6500-0000-7b17-96a2-e0b9e49654cc', 'secondary', NULL),
    ('019a6500-0000-7b0f-a5e4-69fffdd268cd', '019a6500-0000-7dd7-9636-097f120cb875', '019a6500-0000-7f72-a81b-1785504e542f', 'avoid', NULL),
    ('019a6500-0000-7f58-b244-fec268e0a634', '019a6500-0000-7dd7-9636-097f120cb875', '019a6500-0000-781e-890c-26a2ab1e860a', 'avoid', NULL),
    ('019a6500-0000-7f54-a763-093a40233152', '019a6500-0000-75c2-be24-3bce2db1c90b', '019a6500-0000-78f8-8142-f3042f1dfc16', 'primary', NULL),
    ('019a6500-0000-77d0-8061-af69faa3a3b5', '019a6500-0000-75c2-be24-3bce2db1c90b', '019a6500-0000-7e21-af35-40d6ecae3d03', 'secondary', NULL),
    ('019a6500-0000-730f-b527-c91fa6db2f9d', '019a6500-0000-75c2-be24-3bce2db1c90b', '019a6500-0000-727a-a588-19eb0d4700b0', 'secondary', NULL),
    ('019a6500-0000-7625-b865-033e554e5c80', '019a6500-0000-75c2-be24-3bce2db1c90b', '019a6500-0000-7c30-a504-026fbecda593', 'avoid', NULL),
    ('019a6500-0000-7101-adee-f1a61e0c806a', '019a6500-0000-75c2-be24-3bce2db1c90b', '019a6500-0000-781e-890c-26a2ab1e860a', 'avoid', NULL),
    ('019a6500-0000-7349-83a5-14cdf2698edb', '019a6500-0000-7a33-a5a1-0f6b963f4787', '019a6500-0000-732e-8684-2fc7e00f2bbe', 'primary', NULL),
    ('019a6500-0000-7cb0-8f76-20a32b9199e6', '019a6500-0000-7a33-a5a1-0f6b963f4787', '019a6500-0000-7c7a-9bee-3a426a776c5b', 'secondary', NULL),
    ('019a6500-0000-7660-b6c9-64f8c72e14ef', '019a6500-0000-7a33-a5a1-0f6b963f4787', '019a6500-0000-7a0f-92f9-998ff032e9e4', 'secondary', NULL),
    ('019a6500-0000-73be-b711-3263f19236cd', '019a6500-0000-7a33-a5a1-0f6b963f4787', '019a6500-0000-7b07-85bc-61da2a3e7fbe', 'avoid', NULL),
    ('019a6500-0000-7172-b049-9959718aea01', '019a6500-0000-7a33-a5a1-0f6b963f4787', '019a6500-0000-7fd7-a705-19bd85dc15fe', 'avoid', NULL),
    ('019a6500-0000-7e94-b0c5-6dde25ac535f', '019a6500-0000-739f-bd64-494f336e80cd', '019a6500-0000-7eaa-9ff1-ddfff78c9536', 'primary', NULL),
    ('019a6500-0000-7586-9351-e57ce071ebbb', '019a6500-0000-739f-bd64-494f336e80cd', '019a6500-0000-73ba-940e-09f83c92c170', 'secondary', NULL),
    ('019a6500-0000-70c4-b73a-6fc3d5034248', '019a6500-0000-739f-bd64-494f336e80cd', '019a6500-0000-7a4c-9287-15371c179746', 'secondary', NULL),
    ('019a6500-0000-7eb5-a13f-408e1ae1d8ef', '019a6500-0000-739f-bd64-494f336e80cd', '019a6500-0000-7d16-ab7a-15287341dca8', 'avoid', NULL),
    ('019a6500-0000-7a9e-84f8-fcebe07121c6', '019a6500-0000-739f-bd64-494f336e80cd', '019a6500-0000-781e-890c-26a2ab1e860a', 'avoid', NULL),
    ('019a6500-0000-7d53-b89f-eb9911311076', '019a6500-0000-79f6-9471-7ab37e5da57b', '019a6500-0000-7b00-8e00-1f670cbb5290', 'primary', NULL),
    ('019a6500-0000-756d-b9d8-0b05b6a17388', '019a6500-0000-79f6-9471-7ab37e5da57b', '019a6500-0000-72b8-96ee-46aa0592bf6f', 'secondary', NULL),
    ('019a6500-0000-7277-b218-8ca9f2511b40', '019a6500-0000-79f6-9471-7ab37e5da57b', '019a6500-0000-7529-bfe2-5b70678c22b5', 'secondary', NULL),
    ('019a6500-0000-7f23-8af5-442fa5dc3245', '019a6500-0000-79f6-9471-7ab37e5da57b', '019a6500-0000-77c5-8918-0ded7d2229c4', 'avoid', NULL),
    ('019a6500-0000-79ab-9cfa-0b9eced0f691', '019a6500-0000-79f6-9471-7ab37e5da57b', '019a6500-0000-77a1-b887-4498c8deba56', 'avoid', NULL),
    ('019a6500-0000-79ba-ba93-7a12a582352a', '019a6500-0000-7258-a9d5-09fc77154fbc', '019a6500-0000-77b9-aec6-aef234a11cc3', 'primary', NULL),
    ('019a6500-0000-7651-809e-63f8cd099ac4', '019a6500-0000-7258-a9d5-09fc77154fbc', '019a6500-0000-773a-a988-5f92c6e793a3', 'secondary', NULL),
    ('019a6500-0000-771d-a06b-85e9d344e1b5', '019a6500-0000-7258-a9d5-09fc77154fbc', '019a6500-0000-755b-a3b4-50b7cf07f5b9', 'secondary', NULL),
    ('019a6500-0000-7d66-a8bd-fb32db312077', '019a6500-0000-7258-a9d5-09fc77154fbc', '019a6500-0000-723c-8281-214be6d86343', 'avoid', NULL),
    ('019a6500-0000-75c1-b7d3-377e2fcd083a', '019a6500-0000-7258-a9d5-09fc77154fbc', '019a6500-0000-781e-890c-26a2ab1e860a', 'avoid', NULL),
    ('019a6500-0000-78c3-8d04-dbc4f0dedf65', '019a6500-0000-74eb-be4a-4814423ecf6c', '019a6500-0000-7fa2-87f7-e6c19227ed93', 'primary', NULL),
    ('019a6500-0000-7773-a0e9-ecf6ee3f129d', '019a6500-0000-74eb-be4a-4814423ecf6c', '019a6500-0000-7e63-8b4d-82def9bb78a1', 'secondary', NULL),
    ('019a6500-0000-79ce-bebd-4f3237c830be', '019a6500-0000-74eb-be4a-4814423ecf6c', '019a6500-0000-73e7-86ca-be67c9017053', 'secondary', NULL),
    ('019a6500-0000-782b-8820-5df0fc8613b3', '019a6500-0000-74eb-be4a-4814423ecf6c', '019a6500-0000-7291-a348-e4da007ee88f', 'avoid', NULL),
    ('019a6500-0000-76fa-88d0-db62177b29e1', '019a6500-0000-74eb-be4a-4814423ecf6c', '019a6500-0000-7fd7-a705-19bd85dc15fe', 'avoid', NULL),
    ('019a6500-0000-7d02-ab2f-120d62f00480', '019a6500-0000-7284-888c-0d323eb04610', '019a6500-0000-778e-aa7b-a8c4aa6746f9', 'primary', NULL),
    ('019a6500-0000-7f8d-af67-0c3b3b70dd74', '019a6500-0000-7284-888c-0d323eb04610', '019a6500-0000-7317-aec5-af6abea2b18e', 'secondary', NULL),
    ('019a6500-0000-7b4b-9b13-6f81880b3970', '019a6500-0000-7284-888c-0d323eb04610', '019a6500-0000-73ee-86fd-cd492f75dc9e', 'secondary', NULL),
    ('019a6500-0000-7d14-b9b0-54a8b2e7bb0f', '019a6500-0000-7284-888c-0d323eb04610', '019a6500-0000-7790-b4f3-dc7c853664aa', 'avoid', NULL),
    ('019a6500-0000-7bf2-8f8f-0a5f250e65d8', '019a6500-0000-7284-888c-0d323eb04610', '019a6500-0000-7234-aa25-b7b08ebc8bd5', 'avoid', NULL),
    ('019a6500-0000-7703-be44-bb454f2bcb35', '019a6500-0000-7680-a395-5f1e6328b5cc', '019a6500-0000-706e-96f9-e89d26a07c6b', 'primary', NULL),
    ('019a6500-0000-78b2-bef7-657855f5949d', '019a6500-0000-7680-a395-5f1e6328b5cc', '019a6500-0000-792f-aadf-6b8d04a243fd', 'secondary', NULL),
    ('019a6500-0000-7e61-b404-09fafdf293b9', '019a6500-0000-7680-a395-5f1e6328b5cc', '019a6500-0000-77c7-b144-e5221bd81ced', 'secondary', NULL),
    ('019a6500-0000-7fe2-9c70-58a55f118c5c', '019a6500-0000-7680-a395-5f1e6328b5cc', '019a6500-0000-72c8-b7bb-575a6ef7abcd', 'avoid', NULL),
    ('019a6500-0000-7139-af18-750d22dac905', '019a6500-0000-7680-a395-5f1e6328b5cc', '019a6500-0000-7920-8c38-16fc7d7483a0', 'avoid', NULL),
    ('019a6500-0000-70d4-8a17-cac811c35535', '019a6500-0000-7826-93be-ba880b20a902', '019a6500-0000-7acb-8d28-4d519b04e698', 'primary', NULL),
    ('019a6500-0000-73f9-b06b-4ad866b050d0', '019a6500-0000-7826-93be-ba880b20a902', '019a6500-0000-7707-ada0-3831f1bc72b6', 'secondary', NULL),
    ('019a6500-0000-7a7a-aee5-c01d82b6e4d8', '019a6500-0000-7826-93be-ba880b20a902', '019a6500-0000-7686-b935-8c49f6ac5aa2', 'secondary', NULL),
    ('019a6500-0000-72d5-8051-ad10768959fc', '019a6500-0000-7826-93be-ba880b20a902', '019a6500-0000-73a0-a576-5ee77b7a275e', 'avoid', NULL),
    ('019a6500-0000-7eda-9c37-8a51f01e5940', '019a6500-0000-7826-93be-ba880b20a902', '019a6500-0000-7695-ac8b-827aaf2926fe', 'avoid', NULL),
    ('019a6500-0000-7b9c-97a9-9e243ae8ab0a', '019a6500-0000-75b1-bb22-afb22c584a64', '019a6500-0000-7f3d-9745-d9ba40bf52f3', 'primary', NULL),
    ('019a6500-0000-717c-af4a-f0fc8af51eac', '019a6500-0000-75b1-bb22-afb22c584a64', '019a6500-0000-7317-9c40-209b67a5857d', 'secondary', NULL),
    ('019a6500-0000-75b5-9dd8-b81cd09284e9', '019a6500-0000-75b1-bb22-afb22c584a64', '019a6500-0000-7bc0-95b4-fd072f2c7804', 'secondary', NULL),
    ('019a6500-0000-7a17-b4c1-036c1be81f73', '019a6500-0000-75b1-bb22-afb22c584a64', '019a6500-0000-7bee-8955-add413976b01', 'avoid', NULL),
    ('019a6500-0000-78c3-9aff-43970bc6bc37', '019a6500-0000-75b1-bb22-afb22c584a64', '019a6500-0000-7092-abfe-14836d6072fb', 'avoid', NULL),
    ('019a6500-0000-7709-8615-67d455c548a9', '019a6500-0000-7f39-b49d-8e438c9bb364', '019a6500-0000-728a-af43-335a353fb31c', 'primary', NULL),
    ('019a6500-0000-786c-9087-096b8e5b1c9c', '019a6500-0000-7f39-b49d-8e438c9bb364', '019a6500-0000-77af-829a-6a6c63578091', 'secondary', NULL),
    ('019a6500-0000-7873-b1af-8c954fac3f61', '019a6500-0000-7f39-b49d-8e438c9bb364', '019a6500-0000-77a8-87ef-188948620694', 'secondary', NULL),
    ('019a6500-0000-7f6a-a6db-bc8cf8bf0454', '019a6500-0000-7f39-b49d-8e438c9bb364', '019a6500-0000-779f-8f43-43a21dd73716', 'avoid', NULL),
    ('019a6500-0000-7edd-a7c6-b8327a96eb7f', '019a6500-0000-7f39-b49d-8e438c9bb364', '019a6500-0000-7bee-8955-add413976b01', 'avoid', NULL),
    ('019a6500-0000-7745-92a5-9a4ebba4c239', '019a6500-0000-7f82-a217-bda9aed38239', '019a6500-0000-7f9f-99e2-51f61d214a74', 'primary', NULL),
    ('019a6500-0000-7924-93c0-0367ecf5f419', '019a6500-0000-7f82-a217-bda9aed38239', '019a6500-0000-7efa-bbcb-6f1c1faef619', 'secondary', NULL),
    ('019a6500-0000-7a5a-8b8d-fc7a12cde49e', '019a6500-0000-7f82-a217-bda9aed38239', '019a6500-0000-72f3-88aa-c5e1bf59858c', 'secondary', NULL),
    ('019a6500-0000-7da0-aec9-b2147c1cabff', '019a6500-0000-7f82-a217-bda9aed38239', '019a6500-0000-7092-8f60-f91bef16014e', 'avoid', NULL),
    ('019a6500-0000-7691-83ef-9a98af0d0ef0', '019a6500-0000-7f82-a217-bda9aed38239', '019a6500-0000-7525-866d-ac34a4fb1ed7', 'avoid', NULL),
    ('019a6500-0000-718f-b019-b0dea3c4b2e7', '019a6500-0000-7b93-8fa0-2accfcfdd6d9', '019a6500-0000-7046-a16d-1c0b36f0aaf0', 'primary', NULL),
    ('019a6500-0000-785c-922a-ed07426b9f10', '019a6500-0000-7b93-8fa0-2accfcfdd6d9', '019a6500-0000-74d0-9920-aee8a42af8ed', 'secondary', NULL),
    ('019a6500-0000-76e5-ae79-d29d653f0c53', '019a6500-0000-7b93-8fa0-2accfcfdd6d9', '019a6500-0000-7177-a425-f1423b712446', 'secondary', NULL),
    ('019a6500-0000-71ee-826e-2d01098ebe9f', '019a6500-0000-7b93-8fa0-2accfcfdd6d9', '019a6500-0000-7b07-85bc-61da2a3e7fbe', 'avoid', NULL),
    ('019a6500-0000-7346-b44a-3ca41879b10a', '019a6500-0000-7b93-8fa0-2accfcfdd6d9', '019a6500-0000-7bee-8955-add413976b01', 'avoid', NULL),
    ('019a6500-0000-72bb-915f-49b218c3bb55', '019a6500-0000-7988-bc9c-c786b2a97e2a', '019a6500-0000-7974-a0b2-5ef14378c55c', 'primary', NULL),
    ('019a6500-0000-7488-b29c-4701dd4a6fa3', '019a6500-0000-7988-bc9c-c786b2a97e2a', '019a6500-0000-7ea5-8d76-ff5c4a8f4609', 'secondary', NULL),
    ('019a6500-0000-7126-b272-8ee5b1e1ef0b', '019a6500-0000-7988-bc9c-c786b2a97e2a', '019a6500-0000-7d01-a3f0-f9e3e95c3b32', 'secondary', NULL),
    ('019a6500-0000-798f-befa-998d3542a9be', '019a6500-0000-7988-bc9c-c786b2a97e2a', '019a6500-0000-7907-9732-b8669ea97868', 'avoid', NULL),
    ('019a6500-0000-7f27-9d12-5d0f83533894', '019a6500-0000-7988-bc9c-c786b2a97e2a', '019a6500-0000-71b9-8047-12e84bdfd553', 'avoid', NULL),
    ('019a6500-0000-7da9-8a4f-55776eb90ce4', '019a6500-0000-74a1-8a06-716442cbe3f6', '019a6500-0000-7970-bb95-4dfab5295df0', 'primary', NULL),
    ('019a6500-0000-74e3-8daa-e23eb4c85637', '019a6500-0000-74a1-8a06-716442cbe3f6', '019a6500-0000-71c9-8e2d-d2060a870ed7', 'secondary', NULL),
    ('019a6500-0000-74b9-a619-f4fa592ca121', '019a6500-0000-74a1-8a06-716442cbe3f6', '019a6500-0000-7033-87b3-8c1c42560558', 'secondary', NULL),
    ('019a6500-0000-787f-9987-4f83c2dcf536', '019a6500-0000-74a1-8a06-716442cbe3f6', '019a6500-0000-79cb-ab2b-aed1a742aadb', 'avoid', NULL),
    ('019a6500-0000-7f8b-af79-d09164d16379', '019a6500-0000-74a1-8a06-716442cbe3f6', '019a6500-0000-790a-9bf2-e7382c29f001', 'avoid', NULL),
    ('019a6500-0000-742e-a3b9-c9f46937ab88', '019a6500-0000-7136-b533-7749359250e5', '019a6500-0000-7aea-b194-64e85b72a37f', 'primary', NULL),
    ('019a6500-0000-77fd-bc17-6cd6700d0cc6', '019a6500-0000-7136-b533-7749359250e5', '019a6500-0000-70b5-ac95-a4005892dff1', 'secondary', NULL),
    ('019a6500-0000-7c59-9f5c-21486bcb527e', '019a6500-0000-7136-b533-7749359250e5', '019a6500-0000-7317-b7bf-73baaa278964', 'secondary', NULL),
    ('019a6500-0000-70f1-98fc-2959b1754937', '019a6500-0000-7136-b533-7749359250e5', '019a6500-0000-70aa-b4a6-1fdcd1c9c075', 'avoid', NULL),
    ('019a6500-0000-7427-9efb-6ea50dcef2cb', '019a6500-0000-7136-b533-7749359250e5', '019a6500-0000-7092-abfe-14836d6072fb', 'avoid', NULL),
    ('019a6500-0000-78ca-98dc-aeabc2b52208', '019a6500-0000-7013-a5c4-19de49103960', '019a6500-0000-7c66-8422-3502a14f8b4f', 'primary', NULL),
    ('019a6500-0000-73db-9c20-d8fa0c2813c8', '019a6500-0000-7013-a5c4-19de49103960', '019a6500-0000-78e3-95b8-16b71a83b966', 'secondary', NULL),
    ('019a6500-0000-72ce-8db2-d3fdabcd1a7c', '019a6500-0000-7013-a5c4-19de49103960', '019a6500-0000-7bcd-b2bf-7aacf1e3bb07', 'secondary', NULL),
    ('019a6500-0000-7231-b2db-10d0a10d35ad', '019a6500-0000-7013-a5c4-19de49103960', '019a6500-0000-714e-aa2b-2238dbf1096a', 'avoid', NULL),
    ('019a6500-0000-73d3-8e31-ebcbfb93928d', '019a6500-0000-7013-a5c4-19de49103960', '019a6500-0000-715c-aa74-1c31b40d1b93', 'avoid', NULL),
    ('019a6500-0000-7b93-9103-3d90ad7d71ba', '019a6500-0000-710b-9e7f-a3f076a47db8', '019a6500-0000-7579-82c1-621043f650a4', 'primary', NULL),
    ('019a6500-0000-7f6e-81cd-1ce018f860c1', '019a6500-0000-710b-9e7f-a3f076a47db8', '019a6500-0000-7446-ab65-5c315d456cf9', 'secondary', NULL),
    ('019a6500-0000-7931-b639-316351553e20', '019a6500-0000-710b-9e7f-a3f076a47db8', '019a6500-0000-74da-8cc6-2f202b5b8514', 'secondary', NULL),
    ('019a6500-0000-700e-a028-9539e3e3e08c', '019a6500-0000-710b-9e7f-a3f076a47db8', '019a6500-0000-70dd-b584-bf394e6f192a', 'avoid', NULL),
    ('019a6500-0000-743d-8a50-6863676e0224', '019a6500-0000-710b-9e7f-a3f076a47db8', '019a6500-0000-715c-aa74-1c31b40d1b93', 'avoid', NULL),
    ('019a6500-0000-7ffd-9c72-7a9b0c53b35e', '019a6500-0000-7279-a44e-11fa1d21e02a', '019a6500-0000-7db0-85fd-f27632a56efc', 'primary', NULL),
    ('019a6500-0000-750a-b867-51b3be5be285', '019a6500-0000-7279-a44e-11fa1d21e02a', '019a6500-0000-770b-886b-5f9802bdf404', 'secondary', NULL),
    ('019a6500-0000-7a69-85b8-9e61790fb620', '019a6500-0000-7279-a44e-11fa1d21e02a', '019a6500-0000-7c5e-a0bb-d6ffec4a2bbc', 'secondary', NULL),
    ('019a6500-0000-7a13-8cf4-d64a8cfaf1a1', '019a6500-0000-7279-a44e-11fa1d21e02a', '019a6500-0000-7766-a2b8-f6660979a21c', 'avoid', NULL),
    ('019a6500-0000-7710-bbdc-880bb843bcaa', '019a6500-0000-7279-a44e-11fa1d21e02a', '019a6500-0000-7bb4-9c73-7a85962812d1', 'avoid', NULL),
    ('019a6500-0000-7bec-9b12-a714197bff94', '019a6500-0000-75ef-b0d5-583af2593e02', '019a6500-0000-78a6-9d70-87ac7dcab939', 'primary', NULL),
    ('019a6500-0000-7bcf-a558-1c3b5a9fb545', '019a6500-0000-75ef-b0d5-583af2593e02', '019a6500-0000-723d-9c43-e13f52e4f239', 'secondary', NULL),
    ('019a6500-0000-78b6-98a9-c09d0983dac3', '019a6500-0000-75ef-b0d5-583af2593e02', '019a6500-0000-7457-b4d9-7321de1e7161', 'secondary', NULL),
    ('019a6500-0000-7a29-99b1-aca7e707bd2e', '019a6500-0000-75ef-b0d5-583af2593e02', '019a6500-0000-75c2-b1cb-090fce6a7b1b', 'avoid', NULL),
    ('019a6500-0000-7510-a255-21a3295f39c6', '019a6500-0000-75ef-b0d5-583af2593e02', '019a6500-0000-79c0-8d63-800c095ad6f2', 'avoid', NULL),
    ('019a6500-0000-72b1-ac99-537785384e56', '019a6500-0000-7ccf-aae9-5dbc419436e9', '019a6500-0000-7385-8484-6525b267e3f9', 'primary', NULL),
    ('019a6500-0000-7b08-b341-690a3563ef2a', '019a6500-0000-7ccf-aae9-5dbc419436e9', '019a6500-0000-7160-826d-55e31a0e380a', 'secondary', NULL),
    ('019a6500-0000-7a75-8f9b-68053fad493c', '019a6500-0000-7ccf-aae9-5dbc419436e9', '019a6500-0000-7e6f-a355-6fdfd90579e1', 'secondary', NULL),
    ('019a6500-0000-7b47-a169-7ff2ac098786', '019a6500-0000-7ccf-aae9-5dbc419436e9', '019a6500-0000-7525-866d-ac34a4fb1ed7', 'avoid', NULL),
    ('019a6500-0000-7cc9-bac1-73ec270fdbf4', '019a6500-0000-7ccf-aae9-5dbc419436e9', '019a6500-0000-7ed7-ad1a-704c8e39e19e', 'avoid', NULL),
    ('019a6500-0000-7c28-a6b5-b7b7907b589e', '019a6500-0000-7a20-96a3-46f0b10badb9', '019a6500-0000-70d6-b0e2-eb1733b98ec2', 'primary', NULL),
    ('019a6500-0000-70ac-bf5d-02777fe6f235', '019a6500-0000-7a20-96a3-46f0b10badb9', '019a6500-0000-7293-b9b3-11c456ba790b', 'secondary', NULL),
    ('019a6500-0000-7070-b256-5e6aa79d783c', '019a6500-0000-7a20-96a3-46f0b10badb9', '019a6500-0000-7bcd-b2bf-7aacf1e3bb07', 'secondary', NULL),
    ('019a6500-0000-7009-aa62-5d102bbeaf9c', '019a6500-0000-7a20-96a3-46f0b10badb9', '019a6500-0000-781a-b79e-8088aaf4e3f3', 'avoid', NULL),
    ('019a6500-0000-7a14-84da-c6406a22d861', '019a6500-0000-7a20-96a3-46f0b10badb9', '019a6500-0000-7f4b-a765-a4e22e69eed0', 'avoid', NULL),
    ('019a6500-0000-70ae-adf4-4232abcbc146', '019a6500-0000-7c26-b666-49b954ef73b5', '019a6500-0000-7bbb-9d02-205760a24b9b', 'primary', NULL),
    ('019a6500-0000-70ba-b04c-28855e13efa2', '019a6500-0000-7c26-b666-49b954ef73b5', '019a6500-0000-768d-b2d1-72a394e4f1a0', 'secondary', NULL),
    ('019a6500-0000-7053-bd52-94ff2ca831ca', '019a6500-0000-7c26-b666-49b954ef73b5', '019a6500-0000-7051-a1bd-41a5252dc467', 'secondary', NULL),
    ('019a6500-0000-7a95-a220-a5d090ff07cb', '019a6500-0000-7c26-b666-49b954ef73b5', '019a6500-0000-750d-8c11-f474ca697159', 'avoid', NULL),
    ('019a6500-0000-7982-8a4f-f174cf1c8060', '019a6500-0000-7c26-b666-49b954ef73b5', '019a6500-0000-7800-818a-a74a60184fbc', 'avoid', NULL),
    ('019a6500-0000-725d-990c-9540867413de', '019a6500-0000-78e7-9c4a-57f623bac127', '019a6500-0000-7e0a-a038-2e1b80dd6db4', 'primary', NULL),
    ('019a6500-0000-712a-87e6-4f9a6acf88ff', '019a6500-0000-78e7-9c4a-57f623bac127', '019a6500-0000-71b6-8a7c-936e14ca3b57', 'secondary', NULL),
    ('019a6500-0000-78a7-abbf-f6cfa87c7b29', '019a6500-0000-78e7-9c4a-57f623bac127', '019a6500-0000-7b82-804a-c59bc156d87f', 'secondary', NULL),
    ('019a6500-0000-7bd8-80bc-cbd9a1b0689a', '019a6500-0000-78e7-9c4a-57f623bac127', '019a6500-0000-7991-89af-47817eac2c08', 'avoid', NULL),
    ('019a6500-0000-78cd-b513-b316157a96a0', '019a6500-0000-78e7-9c4a-57f623bac127', '019a6500-0000-740b-96f0-50921ecd7c37', 'avoid', NULL),
    ('019a6500-0000-7c8e-a01e-62370010d126', '019a6500-0000-79ee-8a9a-5d6196bfb8a4', '019a6500-0000-7b48-9405-5f2550fd04da', 'primary', NULL),
    ('019a6500-0000-7031-b704-ff3ab2a840dd', '019a6500-0000-79ee-8a9a-5d6196bfb8a4', '019a6500-0000-759c-867f-4b9a76d1f38a', 'secondary', NULL),
    ('019a6500-0000-7f35-b03f-950641892f14', '019a6500-0000-79ee-8a9a-5d6196bfb8a4', '019a6500-0000-76a1-a961-f91d4a3b5c3a', 'secondary', NULL),
    ('019a6500-0000-74a1-99fa-f87e3a7e4d76', '019a6500-0000-79ee-8a9a-5d6196bfb8a4', '019a6500-0000-76e2-88ac-2ba412e16fcb', 'avoid', NULL),
    ('019a6500-0000-7116-a5a3-d83035ef8b36', '019a6500-0000-79ee-8a9a-5d6196bfb8a4', '019a6500-0000-77db-9d36-34bfc90a5ad4', 'avoid', NULL);

INSERT OR IGNORE INTO style_preset_vocab_items (id, preset_id, content) VALUES
    ('019a6500-0000-7357-b2d5-cff9c1ad189a', '019a6500-0000-7649-9191-8d23bb0272ef', 'động từ tĩnh'),
    ('019a6500-0000-7158-8a58-49f17cb7f27a', '019a6500-0000-7649-9191-8d23bb0272ef', 'từ ám chỉ'),
    ('019a6500-0000-73f7-a28c-35fab0640feb', '019a6500-0000-7649-9191-8d23bb0272ef', 'từ mang tính ẩn dụ nhẹ'),
    ('019a6500-0000-79bb-8a21-f69535936f0c', '019a6500-0000-73bf-8965-43db5c54bb13', 'động từ mạnh'),
    ('019a6500-0000-7e8f-8ffd-3e74e6e5b705', '019a6500-0000-73bf-8965-43db5c54bb13', 'từ chỉ số lượng'),
    ('019a6500-0000-7f02-b363-945919110551', '019a6500-0000-73bf-8965-43db5c54bb13', 'từ tượng thanh hỗn tạp'),
    ('019a6500-0000-7fb0-84ea-d11997546dae', '019a6500-0000-778d-abea-bc0c279898e7', 'động từ chuyển động'),
    ('019a6500-0000-7e4f-8f67-bb7c5ffca810', '019a6500-0000-778d-abea-bc0c279898e7', 'từ chỉ tốc độ'),
    ('019a6500-0000-7654-95d7-783a935a0987', '019a6500-0000-778d-abea-bc0c279898e7', 'từ tượng thanh'),
    ('019a6500-0000-7f71-9784-9e5c519ef919', '019a6500-0000-7ba6-9823-fbd6d26f435c', 'động từ mạnh'),
    ('019a6500-0000-7c69-8656-0c93ca2d7882', '019a6500-0000-7ba6-9823-fbd6d26f435c', 'từ tượng thanh'),
    ('019a6500-0000-70f6-a198-df8e93be0d3a', '019a6500-0000-7120-a6d1-779128366e02', 'động từ chính xác'),
    ('019a6500-0000-7b86-b4cf-82cdf1ec5bf0', '019a6500-0000-7120-a6d1-779128366e02', 'thuật ngữ kỹ thuật'),
    ('019a6500-0000-7924-a9ba-1b665611e164', '019a6500-0000-7120-a6d1-779128366e02', 'từ miêu tả chuyển động nhỏ'),
    ('019a6500-0000-7088-b27a-c3e9822a8859', '019a6500-0000-7305-ad33-f264f3fb90a1', 'động từ chạy, ẩn nấp'),
    ('019a6500-0000-7d43-920c-6b8c12d1aa1e', '019a6500-0000-7305-ad33-f264f3fb90a1', 'từ chỉ thời gian'),
    ('019a6500-0000-718e-9285-3746d7fb7bfd', '019a6500-0000-74f6-bd80-6fb76004f436', 'động từ dứt khoát'),
    ('019a6500-0000-7031-bcd5-805363a64988', '019a6500-0000-74f6-bd80-6fb76004f436', 'từ ngắn gọn'),
    ('019a6500-0000-7b27-a876-18dbe2d25787', '019a6500-0000-74f6-bd80-6fb76004f436', 'từ mang tính kết thúc'),
    ('019a6500-0000-7b17-9af8-bcd758d065db', '019a6500-0000-7be9-a265-fd4d937cdf70', 'từ chỉ quan sát'),
    ('019a6500-0000-7a09-aa68-69b226289184', '019a6500-0000-7be9-a265-fd4d937cdf70', 'động từ lén lút'),
    ('019a6500-0000-7fed-9d20-4cad658dc186', '019a6500-0000-7be9-a265-fd4d937cdf70', 'thuật ngữ trinh sát'),
    ('019a6500-0000-78c4-849e-3ff0baae1b20', '019a6500-0000-76df-89bd-0a3cec6bf787', 'động từ nặng'),
    ('019a6500-0000-747b-80a2-5dc270f1bf79', '019a6500-0000-76df-89bd-0a3cec6bf787', 'từ gợi cảm xúc'),
    ('019a6500-0000-79fc-9514-e29ca851ae85', '019a6500-0000-76df-89bd-0a3cec6bf787', 'ẩn dụ'),
    ('019a6500-0000-7787-a8e7-f9549e7c89a9', '019a6500-0000-7071-b143-4558d813cb0b', 'từ cảm giác cơ thể'),
    ('019a6500-0000-78ae-8923-725abbe159a7', '019a6500-0000-7071-b143-4558d813cb0b', 'từ chỉ thời tiết, địa hình'),
    ('019a6500-0000-7564-8823-c731a544a4c0', '019a6500-0000-7071-b143-4558d813cb0b', 'động từ hành động cụ thể'),
    ('019a6500-0000-7c11-b97b-cbd307a53005', '019a6500-0000-757d-9a4e-3784abbe108a', 'thuật ngữ kỹ thuật'),
    ('019a6500-0000-7148-9937-fcb908993a83', '019a6500-0000-757d-9a4e-3784abbe108a', 'động từ hướng dẫn'),
    ('019a6500-0000-7c10-b190-5428bd2c9719', '019a6500-0000-757d-9a4e-3784abbe108a', 'từ chỉ động tác'),
    ('019a6500-0000-7dc5-8c95-9c456baed883', '019a6500-0000-7432-9d99-9a3a3bf6d095', 'thuật ngữ quân sự'),
    ('019a6500-0000-751a-ae1f-1e7bdc72e0f5', '019a6500-0000-7432-9d99-9a3a3bf6d095', 'từ chỉ chiến thuật'),
    ('019a6500-0000-77a5-abb1-a7401d1ee757', '019a6500-0000-7432-9d99-9a3a3bf6d095', 'từ chỉ quy mô'),
    ('019a6500-0000-7fbc-860a-f5649b250b20', '019a6500-0000-72a2-92cf-2095bad1f02e', 'từ miêu tả sự kỳ lạ'),
    ('019a6500-0000-7662-8e51-bd40cc4e993e', '019a6500-0000-72a2-92cf-2095bad1f02e', 'ẩn dụ so sánh với điều quen thuộc'),
    ('019a6500-0000-73bc-b8f6-8ee74f7c4ed0', '019a6500-0000-72a2-92cf-2095bad1f02e', 'động từ quan sát'),
    ('019a6500-0000-7f7b-a5f9-29d0bcd9608e', '019a6500-0000-7028-8c25-e1ddb07d06fc', 'thuật ngữ địa hình'),
    ('019a6500-0000-74fe-a9b8-2399fd453fc7', '019a6500-0000-7028-8c25-e1ddb07d06fc', 'từ chỉ sự khai mở, hiển lộ'),
    ('019a6500-0000-7e59-bd6f-a2dcdbf117f9', '019a6500-0000-7028-8c25-e1ddb07d06fc', 'miêu tả màu sắc bản đồ'),
    ('019a6500-0000-7277-b862-23eae20e044f', '019a6500-0000-72ff-940f-10282ee4595d', 'động từ di chuyển'),
    ('019a6500-0000-72d5-8bc5-590051bc4f88', '019a6500-0000-72ff-940f-10282ee4595d', 'thuật ngữ định hướng'),
    ('019a6500-0000-75e3-a383-f8b4693cba07', '019a6500-0000-72ff-940f-10282ee4595d', 'từ miêu tả địa hình'),
    ('019a6500-0000-7b09-bb8f-fcb0e83a144f', '019a6500-0000-750e-b05f-2308b8e27150', 'thuật ngữ cổ học'),
    ('019a6500-0000-7317-b7a0-497b74f93e3a', '019a6500-0000-750e-b05f-2308b8e27150', 'từ chỉ sự xuống cấp, hoang tàn'),
    ('019a6500-0000-7b93-9a9c-104d86e85033', '019a6500-0000-750e-b05f-2308b8e27150', 'ẩn dụ bí mật chôn vùi'),
    ('019a6500-0000-7993-a169-f0335affe089', '019a6500-0000-7bc4-bc5f-6b653c1e6ab1', 'từ ngữ săn tìm'),
    ('019a6500-0000-7536-8284-1040f6524f63', '019a6500-0000-7bc4-bc5f-6b653c1e6ab1', 'thuật ngữ giải đố'),
    ('019a6500-0000-7dc6-80f3-16e1b172ecf3', '019a6500-0000-7bc4-bc5f-6b653c1e6ab1', 'động từ hành động nhanh'),
    ('019a6500-0000-7b05-a969-5f97858efdd0', '019a6500-0000-7ed3-a13e-b3d6813578f6', 'thuật ngữ nguyên liệu'),
    ('019a6500-0000-7b32-b766-7a0169b0413a', '019a6500-0000-7ed3-a13e-b3d6813578f6', 'động từ chế tác'),
    ('019a6500-0000-7ad7-b6cc-88bc48c10fb7', '019a6500-0000-7ed3-a13e-b3d6813578f6', 'từ chỉ sự biến đổi'),
    ('019a6500-0000-7531-a9b1-1d53cfdb8b59', '019a6500-0000-7226-969b-173b38137736', 'động từ biến đổi'),
    ('019a6500-0000-78a4-8552-ae21689cc23b', '019a6500-0000-7226-969b-173b38137736', 'ẩn dụ sức mạnh'),
    ('019a6500-0000-70fa-9e8f-b8f944dccf0d', '019a6500-0000-7226-969b-173b38137736', 'từ chỉ vượt giới hạn'),
    ('019a6500-0000-7ad8-b4e3-90f4d98ed7a1', '019a6500-0000-7857-bbe4-22949902b0e1', 'thuật ngữ công nghệ'),
    ('019a6500-0000-735e-afdb-3d1f45095ec6', '019a6500-0000-7857-bbe4-22949902b0e1', 'động từ xâm nhập'),
    ('019a6500-0000-77e8-9ac4-c7a563e113ed', '019a6500-0000-7857-bbe4-22949902b0e1', 'từ chỉ tốc độ xử lý'),
    ('019a6500-0000-700a-8e14-c74919db17cb', '019a6500-0000-7a0d-87cc-c8f7d8826db8', 'từ ngữ nghi thức cổ'),
    ('019a6500-0000-7f01-bf39-c4b587a8450f', '019a6500-0000-7a0d-87cc-c8f7d8826db8', 'ẩn dụ giới hạn giữa hai thế giới'),
    ('019a6500-0000-7fcd-9773-06a77e1dd74d', '019a6500-0000-7a0d-87cc-c8f7d8826db8', 'động từ triệu gọi'),
    ('019a6500-0000-7003-86f1-c6ad2cef485e', '019a6500-0000-7cf5-aedc-d78a83018bc5', 'từ chỉ sự lặp lại'),
    ('019a6500-0000-763e-ab69-be13fa8b1528', '019a6500-0000-7cf5-aedc-d78a83018bc5', 'ẩn dụ bàn tay thời gian'),
    ('019a6500-0000-7a46-ad24-6140586fe273', '019a6500-0000-7cf5-aedc-d78a83018bc5', 'từ ngữ số phận'),
    ('019a6500-0000-7ef2-8106-0b5b1430a3e1', '019a6500-0000-780b-8831-d89b6b3e40fc', 'thuật ngữ pháp lý'),
    ('019a6500-0000-7b2f-abb9-9f9562e86fc2', '019a6500-0000-780b-8831-d89b6b3e40fc', 'con số, thời gian, địa điểm'),
    ('019a6500-0000-7c8b-99cf-28e7ae3d0eb0', '019a6500-0000-780b-8831-d89b6b3e40fc', 'từ so sánh, đối chiếu'),
    ('019a6500-0000-7ae5-bae3-332d583dddaf', '019a6500-0000-7fba-ab1e-aa10ad020bc9', 'từ miêu tả chi tiết'),
    ('019a6500-0000-73f4-8f2c-5579a4d9a915', '019a6500-0000-7fba-ab1e-aa10ad020bc9', 'thuật ngữ pháp y / khoa học'),
    ('019a6500-0000-7e53-93db-a472bc3377cc', '019a6500-0000-7fba-ab1e-aa10ad020bc9', 'động từ quan sát'),
    ('019a6500-0000-7390-a395-aff3a64753f1', '019a6500-0000-733d-b3f0-5ec9336c80e6', 'từ lập luận (do đó, như vậy)'),
    ('019a6500-0000-7566-9ca8-003c7952b1c4', '019a6500-0000-733d-b3f0-5ec9336c80e6', 'thuật ngữ logic'),
    ('019a6500-0000-71a0-b964-6800d1743881', '019a6500-0000-733d-b3f0-5ec9336c80e6', 'phép loại suy'),
    ('019a6500-0000-7322-97a2-75a88ae10e93', '019a6500-0000-71cb-9ba7-6be304142484', 'từ chỉ sự lén lút'),
    ('019a6500-0000-7667-bf28-61ebb8586e98', '019a6500-0000-71cb-9ba7-6be304142484', 'miêu tả ngoại hình giả'),
    ('019a6500-0000-72dc-aedd-950a30946ac1', '019a6500-0000-71cb-9ba7-6be304142484', 'động từ nhập vai'),
    ('019a6500-0000-738b-8051-ac2731446a33', '019a6500-0000-7c1e-bbd5-583db9dfa7fd', 'câu hỏi dồn dập'),
    ('019a6500-0000-721c-b2f1-68eca3dab295', '019a6500-0000-7c1e-bbd5-583db9dfa7fd', 'từ gây áp lực'),
    ('019a6500-0000-7864-b7f2-1b8fa7a19df0', '019a6500-0000-7c1e-bbd5-583db9dfa7fd', 'lặp lại ý để gài bẫy'),
    ('019a6500-0000-74d8-81ce-018087e24c20', '019a6500-0000-7b95-8330-e53c472be4a4', 'từ ngữ mạnh, dứt khoát'),
    ('019a6500-0000-75d1-aa75-089aec804e3b', '019a6500-0000-7b95-8330-e53c472be4a4', 'câu nói đanh thép'),
    ('019a6500-0000-72f8-9b86-14dd4eab6a73', '019a6500-0000-7b95-8330-e53c472be4a4', 'sự tương phản (hoá ra, thì ra)'),
    ('019a6500-0000-7340-975c-1714d91be9b9', '019a6500-0000-7dd7-9636-097f120cb875', 'từ ngữ bóng gió, ẩn ý'),
    ('019a6500-0000-7a06-811d-ecb2eb5550ea', '019a6500-0000-7dd7-9636-097f120cb875', 'thuật ngữ quyền lực'),
    ('019a6500-0000-7d6c-8271-d475b3c14ea4', '019a6500-0000-7dd7-9636-097f120cb875', 'động từ dàn xếp'),
    ('019a6500-0000-7e28-b5e4-05e143ac1c34', '019a6500-0000-75c2-be24-3bce2db1c90b', 'thuật ngữ hoàng gia'),
    ('019a6500-0000-7e5e-bd7b-1bbdde830e4c', '019a6500-0000-75c2-be24-3bce2db1c90b', 'ẩn dụ quyền lực'),
    ('019a6500-0000-7c82-b004-14a31dcbb2b1', '019a6500-0000-75c2-be24-3bce2db1c90b', 'từ trang trọng'),
    ('019a6500-0000-7cde-8395-e1a13fac4ecb', '019a6500-0000-7a33-a5a1-0f6b963f4787', 'từ ngữ mất mát quê hương'),
    ('019a6500-0000-73ff-901e-74b29a967cb8', '019a6500-0000-7a33-a5a1-0f6b963f4787', 'ẩn dụ rễ bị nhổ'),
    ('019a6500-0000-7f71-aef4-765d709d7a4f', '019a6500-0000-7a33-a5a1-0f6b963f4787', 'động từ rời bỏ'),
    ('019a6500-0000-798c-a0a2-bb5409344e86', '019a6500-0000-739f-bd64-494f336e80cd', 'thuật ngữ đàm phán'),
    ('019a6500-0000-70a0-9651-23dde5489c32', '019a6500-0000-739f-bd64-494f336e80cd', 'từ ngữ lập luận, nhượng bộ'),
    ('019a6500-0000-7d16-85f9-71557a94f2de', '019a6500-0000-739f-bd64-494f336e80cd', 'động từ mặc cả'),
    ('019a6500-0000-70f6-a163-f88411d32b5a', '019a6500-0000-79f6-9471-7ab37e5da57b', 'từ ngữ kêu gọi, hô hào'),
    ('019a6500-0000-79dc-88dc-288656855a2e', '019a6500-0000-79f6-9471-7ab37e5da57b', 'ẩn dụ tập thể'),
    ('019a6500-0000-7897-9904-f7d8d6f7fba4', '019a6500-0000-79f6-9471-7ab37e5da57b', 'động từ hành động'),
    ('019a6500-0000-7db9-896b-8b98c1977cbe', '019a6500-0000-7258-a9d5-09fc77154fbc', 'thuật ngữ pháp lý'),
    ('019a6500-0000-7196-8cd2-716ab85ea992', '019a6500-0000-7258-a9d5-09fc77154fbc', 'từ ngữ buộc tội / bào chữa'),
    ('019a6500-0000-7fb6-9985-04f87ef93133', '019a6500-0000-7258-a9d5-09fc77154fbc', 'lập luận chứng cứ'),
    ('019a6500-0000-7955-99c8-c5b507bc0229', '019a6500-0000-74eb-be4a-4814423ecf6c', 'từ ngữ đối lập (tôi đã tin – hóa ra)'),
    ('019a6500-0000-76f4-94f0-c335b3c8babf', '019a6500-0000-74eb-be4a-4814423ecf6c', 'động từ mạnh chỉ sự đâm thủng'),
    ('019a6500-0000-7523-82f1-6ef8c1e7ace4', '019a6500-0000-74eb-be4a-4814423ecf6c', 'tính từ về sự sụp đổ niềm tin'),
    ('019a6500-0000-7793-b9dd-8e61d8143408', '019a6500-0000-7284-888c-0d323eb04610', 'từ ngữ về sự phân mảnh, tan rã'),
    ('019a6500-0000-7393-826b-6d31c8599db1', '019a6500-0000-7284-888c-0d323eb04610', 'câu hỏi tu từ về bản thân'),
    ('019a6500-0000-74b6-b417-09a456440c5a', '019a6500-0000-7284-888c-0d323eb04610', 'ẩn dụ cho lốt và thực chất'),
    ('019a6500-0000-7329-8f29-5bebdc8ac11e', '019a6500-0000-7680-a395-5f1e6328b5cc', 'từ triết lý, nội quan'),
    ('019a6500-0000-7f75-aba0-9858e4eed36f', '019a6500-0000-7680-a395-5f1e6328b5cc', 'so sánh ẩn dụ về trọng lượng (gánh nặng)'),
    ('019a6500-0000-793c-9d97-40bc74b1981b', '019a6500-0000-7680-a395-5f1e6328b5cc', 'động từ chỉ sự đấu tranh tinh thần'),
    ('019a6500-0000-7d17-ae53-f3c49559ebc7', '019a6500-0000-7826-93be-ba880b20a902', 'động từ gây ám ảnh'),
    ('019a6500-0000-7979-9aa4-71a190165915', '019a6500-0000-7826-93be-ba880b20a902', 'tính từ về sự mất kiểm soát'),
    ('019a6500-0000-7201-86bb-34b5be9f4576', '019a6500-0000-7826-93be-ba880b20a902', 'từ tượng thanh cho âm thanh ám ảnh'),
    ('019a6500-0000-7f42-b3e4-442d5be5f015', '019a6500-0000-75b1-bb22-afb22c584a64', 'từ ngữ về sự chấp nhận'),
    ('019a6500-0000-76a6-a425-a208c53ca699', '019a6500-0000-75b1-bb22-afb22c584a64', 'ẩn dụ về ánh sáng, sự tự do, sự chuộc lỗi'),
    ('019a6500-0000-75f6-a7f5-75925f35b99c', '019a6500-0000-75b1-bb22-afb22c584a64', 'động từ chỉ sự từ bỏ và buông tay'),
    ('019a6500-0000-7b4d-8310-3b7c97031f6e', '019a6500-0000-7f39-b49d-8e438c9bb364', 'từ ngữ ẩn dụ tình cảm'),
    ('019a6500-0000-7112-8e32-54a968c7646a', '019a6500-0000-7f39-b49d-8e438c9bb364', 'động từ bộc lộ cảm xúc'),
    ('019a6500-0000-7b06-afb4-40726156799c', '019a6500-0000-7f39-b49d-8e438c9bb364', 'từ chỉ sự do dự và quyết tâm'),
    ('019a6500-0000-7862-969f-0cd347b35d4f', '019a6500-0000-7f82-a217-bda9aed38239', 'từ miêu tả sinh hoạt'),
    ('019a6500-0000-7fe3-a250-928b2cb0b1d2', '019a6500-0000-7f82-a217-bda9aed38239', 'tính từ nhẹ nhàng'),
    ('019a6500-0000-7bfa-9432-60fae3b5cea7', '019a6500-0000-7f82-a217-bda9aed38239', 'câu hỏi quan tâm'),
    ('019a6500-0000-7507-9ff6-4cda7e222978', '019a6500-0000-7b93-8fa0-2accfcfdd6d9', 'từ ngữ về nỗi đau'),
    ('019a6500-0000-73c1-85ec-d797fd5fdf76', '019a6500-0000-7b93-8fa0-2accfcfdd6d9', 'động từ chỉ sự rạn vỡ'),
    ('019a6500-0000-795c-ae45-3151545457a0', '019a6500-0000-7b93-8fa0-2accfcfdd6d9', 'tính từ u ám'),
    ('019a6500-0000-77fe-a213-5fcee34e82ae', '019a6500-0000-7988-bc9c-c786b2a97e2a', 'câu cáo buộc'),
    ('019a6500-0000-728a-a056-c0a29307a16f', '019a6500-0000-7988-bc9c-c786b2a97e2a', 'từ ngữ sắc bén'),
    ('019a6500-0000-7099-9deb-543ed3c3f5b4', '019a6500-0000-7988-bc9c-c786b2a97e2a', 'tính từ so sánh (anh ta hơn tôi?)'),
    ('019a6500-0000-7456-8abe-c0dc0d2a0c44', '019a6500-0000-74a1-8a06-716442cbe3f6', 'từ gợi nhớ quá khứ'),
    ('019a6500-0000-79d3-bd76-766483e562a2', '019a6500-0000-74a1-8a06-716442cbe3f6', 'tính từ chỉ cảm xúc dâng trào'),
    ('019a6500-0000-7eb3-8dbc-7cffe6b347fd', '019a6500-0000-74a1-8a06-716442cbe3f6', 'so sánh với ký ức'),
    ('019a6500-0000-7bfa-bbd7-0045caa206f7', '019a6500-0000-7136-b533-7749359250e5', 'từ ngữ về sự gắn kết'),
    ('019a6500-0000-7412-81bf-c71c32152e3c', '019a6500-0000-7136-b533-7749359250e5', 'ẩn dụ cho tương lai'),
    ('019a6500-0000-7350-90f1-31ea7d6149fc', '019a6500-0000-7136-b533-7749359250e5', 'tính từ long trọng, tôn kính'),
    ('019a6500-0000-7cb3-898f-c038ca9effb1', '019a6500-0000-7013-a5c4-19de49103960', 'động từ chỉ hành động quen thuộc'),
    ('019a6500-0000-7e9f-9623-05190918dcf7', '019a6500-0000-7013-a5c4-19de49103960', 'tính từ đời thường (ướt, ấm, bừa bộn)'),
    ('019a6500-0000-78a4-9efd-adfcf970f548', '019a6500-0000-7013-a5c4-19de49103960', 'từ tượng thanh nhẹ nhàng'),
    ('019a6500-0000-74d1-bd87-de61c48a0bc4', '019a6500-0000-710b-9e7f-a3f076a47db8', 'từ miêu tả món ăn'),
    ('019a6500-0000-7d9f-8a2b-5ef949a17aa6', '019a6500-0000-710b-9e7f-a3f076a47db8', 'từ tượng thanh vui vẻ'),
    ('019a6500-0000-72f8-bc98-2e228fe80761', '019a6500-0000-710b-9e7f-a3f076a47db8', 'động từ mời mọc'),
    ('019a6500-0000-75dc-96c4-43a97a2b6b6e', '019a6500-0000-7279-a44e-11fa1d21e02a', 'từ miêu tả lễ hội'),
    ('019a6500-0000-7cad-b5e5-4a82f3835d90', '019a6500-0000-7279-a44e-11fa1d21e02a', 'từ tượng thanh sôi động'),
    ('019a6500-0000-72f5-913b-c776057d23b4', '019a6500-0000-7279-a44e-11fa1d21e02a', 'động từ tham gia'),
    ('019a6500-0000-743f-bff7-2d36e341080f', '019a6500-0000-75ef-b0d5-583af2593e02', 'từ ngữ hài hước'),
    ('019a6500-0000-7e31-a138-2f61d48c0a74', '019a6500-0000-75ef-b0d5-583af2593e02', 'động từ hành động nhanh'),
    ('019a6500-0000-770f-8338-f7c207f167be', '019a6500-0000-75ef-b0d5-583af2593e02', 'từ tượng thanh chấn động'),
    ('019a6500-0000-724a-bc47-e2944d80f3f0', '019a6500-0000-7ccf-aae9-5dbc419436e9', 'từ miêu tả hàng hóa'),
    ('019a6500-0000-7c09-b3dc-d5d36da12d8c', '019a6500-0000-7ccf-aae9-5dbc419436e9', 'từ mặc cả'),
    ('019a6500-0000-7c6e-bc29-65a0794148f7', '019a6500-0000-7ccf-aae9-5dbc419436e9', 'động từ lựa chọn'),
    ('019a6500-0000-7279-8c26-e26256ae5a17', '019a6500-0000-7a20-96a3-46f0b10badb9', 'từ ngữ êm dịu'),
    ('019a6500-0000-7ec5-8feb-afcf462f2436', '019a6500-0000-7a20-96a3-46f0b10badb9', 'ẩn dụ chữa lành'),
    ('019a6500-0000-7ab4-8988-6d887b46b721', '019a6500-0000-7a20-96a3-46f0b10badb9', 'động từ buông lỏng'),
    ('019a6500-0000-7563-9847-ca8c52926d57', '019a6500-0000-7c26-b666-49b954ef73b5', 'từ chỉ sự trôi chảy thời gian'),
    ('019a6500-0000-7db8-a0b3-ef2304440c4b', '019a6500-0000-7c26-b666-49b954ef73b5', 'ẩn dụ quá khứ – hiện tại'),
    ('019a6500-0000-7b57-af15-efb64b4c3a77', '019a6500-0000-7c26-b666-49b954ef73b5', 'động từ chuyển giao'),
    ('019a6500-0000-7e09-b690-db70a076d621', '019a6500-0000-78e7-9c4a-57f623bac127', 'động từ di chuyển'),
    ('019a6500-0000-7acc-8d85-a452741302dc', '019a6500-0000-78e7-9c4a-57f623bac127', 'từ miêu tả địa danh'),
    ('019a6500-0000-75be-82e0-3eba635da01b', '019a6500-0000-78e7-9c4a-57f623bac127', 'từ tượng thanh chuyển cảnh'),
    ('019a6500-0000-7fff-b17b-a17c16f5b71e', '019a6500-0000-79ee-8a9a-5d6196bfb8a4', 'từ chỉ sự trì hoãn'),
    ('019a6500-0000-7550-9cc8-957d868a6383', '019a6500-0000-79ee-8a9a-5d6196bfb8a4', 'động từ chờ đợi'),
    ('019a6500-0000-7f25-98c2-964afea27b2a', '019a6500-0000-79ee-8a9a-5d6196bfb8a4', 'ẩn dụ thời gian chậm lại');

INSERT OR IGNORE INTO style_preset_focus_items (id, preset_id, content) VALUES
    ('019a6500-0000-7474-9e25-bac5f1848bdc', '019a6500-0000-7649-9191-8d23bb0272ef', 'thời điểm'),
    ('019a6500-0000-7ea4-a4a0-a9e92f179a80', '019a6500-0000-7649-9191-8d23bb0272ef', 'góc nhìn'),
    ('019a6500-0000-788b-820e-fee34ac0f45a', '019a6500-0000-7649-9191-8d23bb0272ef', 'bóng tối'),
    ('019a6500-0000-7806-b86c-75427d9df963', '019a6500-0000-7649-9191-8d23bb0272ef', 'sự im lặng'),
    ('019a6500-0000-7c02-aefd-63b3bb442ab7', '019a6500-0000-73bf-8965-43db5c54bb13', 'tương tác nhóm'),
    ('019a6500-0000-7d85-bf5f-6a961ebae8af', '019a6500-0000-73bf-8965-43db5c54bb13', 'hỗn loạn'),
    ('019a6500-0000-7c32-9eb6-812620aca578', '019a6500-0000-73bf-8965-43db5c54bb13', 'lợi thế chiến thuật tổng thể'),
    ('019a6500-0000-7b12-a0dd-fbd4b84a4a93', '019a6500-0000-778d-abea-bc0c279898e7', 'quãng đường'),
    ('019a6500-0000-7f24-8b32-7f60f0dc9b51', '019a6500-0000-778d-abea-bc0c279898e7', 'vật cản'),
    ('019a6500-0000-75b4-babe-ed553a91cc11', '019a6500-0000-778d-abea-bc0c279898e7', 'khoảng cách'),
    ('019a6500-0000-739b-9b35-58502212a411', '019a6500-0000-778d-abea-bc0c279898e7', 'hơi thở'),
    ('019a6500-0000-7623-9ee1-b5551987e7ec', '019a6500-0000-7ba6-9823-fbd6d26f435c', 'chuyển động'),
    ('019a6500-0000-7a53-b1fb-8a848f812e8b', '019a6500-0000-7ba6-9823-fbd6d26f435c', 'va chạm'),
    ('019a6500-0000-7a4b-83ff-a9830ca5a47e', '019a6500-0000-7ba6-9823-fbd6d26f435c', 'phản xạ'),
    ('019a6500-0000-7f82-98d0-e51d0f22f262', '019a6500-0000-7120-a6d1-779128366e02', 'thế đứng'),
    ('019a6500-0000-7efd-93e1-ca827e47ad63', '019a6500-0000-7120-a6d1-779128366e02', 'ánh mắt'),
    ('019a6500-0000-7e11-a05a-f3880b2c8818', '019a6500-0000-7120-a6d1-779128366e02', 'hơi thở'),
    ('019a6500-0000-79ca-976b-fbeabfc3bdb9', '019a6500-0000-7120-a6d1-779128366e02', 'nhịp độ'),
    ('019a6500-0000-779c-86e5-326305a2a759', '019a6500-0000-7305-ad33-f264f3fb90a1', 'lối thoát'),
    ('019a6500-0000-765d-8ecf-65d5fce8e7c7', '019a6500-0000-7305-ad33-f264f3fb90a1', 'sự truy đuổi'),
    ('019a6500-0000-7b9b-b3f9-978b2185f74b', '019a6500-0000-7305-ad33-f264f3fb90a1', 'chướng ngại'),
    ('019a6500-0000-7ed5-8c67-ab04ba1f12e1', '019a6500-0000-7305-ad33-f264f3fb90a1', 'thời gian'),
    ('019a6500-0000-7dec-8fde-ad89f956f745', '019a6500-0000-74f6-bd80-6fb76004f436', 'khoảnh khắc'),
    ('019a6500-0000-7d98-8f43-9f19da9c50dc', '019a6500-0000-74f6-bd80-6fb76004f436', 'điểm tiếp xúc'),
    ('019a6500-0000-773d-93e7-b9dd6de35757', '019a6500-0000-74f6-bd80-6fb76004f436', 'kết thúc'),
    ('019a6500-0000-7c7b-8813-530f8a2eb9c3', '019a6500-0000-74f6-bd80-6fb76004f436', 'hệ quả'),
    ('019a6500-0000-7444-814e-8486b0017780', '019a6500-0000-7be9-a265-fd4d937cdf70', 'quan sát'),
    ('019a6500-0000-739a-9834-72eb02500116', '019a6500-0000-7be9-a265-fd4d937cdf70', 'ngụy trang'),
    ('019a6500-0000-75fe-9e79-74826fd6e23a', '019a6500-0000-7be9-a265-fd4d937cdf70', 'đường đi'),
    ('019a6500-0000-7c9c-b8e1-b84ea4ea7dd8', '019a6500-0000-7be9-a265-fd4d937cdf70', 'lịch trình'),
    ('019a6500-0000-782b-9c46-8c15f5290d28', '019a6500-0000-76df-89bd-0a3cec6bf787', 'ánh mắt'),
    ('019a6500-0000-7501-b711-430151689db2', '019a6500-0000-76df-89bd-0a3cec6bf787', 'khoảnh khắc'),
    ('019a6500-0000-75ed-939e-337a42181a87', '019a6500-0000-76df-89bd-0a3cec6bf787', 'hậu quả'),
    ('019a6500-0000-7074-9a44-2833d45fae25', '019a6500-0000-76df-89bd-0a3cec6bf787', 'quyết định'),
    ('019a6500-0000-7357-a982-ea0929cb805f', '019a6500-0000-7071-b143-4558d813cb0b', 'môi trường'),
    ('019a6500-0000-7406-9516-5653eafecf0b', '019a6500-0000-7071-b143-4558d813cb0b', 'cơ thể'),
    ('019a6500-0000-7c59-aa7d-1e3f083bcb0a', '019a6500-0000-7071-b143-4558d813cb0b', 'mệt mỏi'),
    ('019a6500-0000-74e0-869a-b34c9e74cbc5', '019a6500-0000-7071-b143-4558d813cb0b', 'vật dụng'),
    ('019a6500-0000-702e-a47d-33020971034a', '019a6500-0000-757d-9a4e-3784abbe108a', 'kỹ thuật'),
    ('019a6500-0000-7ed4-af58-c398676aabbc', '019a6500-0000-757d-9a4e-3784abbe108a', 'sửa sai'),
    ('019a6500-0000-73e2-99aa-6ca195511ddc', '019a6500-0000-757d-9a4e-3784abbe108a', 'lặp lại'),
    ('019a6500-0000-7ca9-8e8c-7022ae1b90ba', '019a6500-0000-757d-9a4e-3784abbe108a', 'cơ thể'),
    ('019a6500-0000-70a9-856c-d06351714ea5', '019a6500-0000-7432-9d99-9a3a3bf6d095', 'chiến lược'),
    ('019a6500-0000-7c16-8c50-26fd08925b43', '019a6500-0000-7432-9d99-9a3a3bf6d095', 'hậu cần'),
    ('019a6500-0000-773d-9b39-00946b51f5f1', '019a6500-0000-7432-9d99-9a3a3bf6d095', 'quy mô'),
    ('019a6500-0000-7d02-83f7-9a549bc8f485', '019a6500-0000-7432-9d99-9a3a3bf6d095', 'thời gian'),
    ('019a6500-0000-77a3-8a49-6a170e1be673', '019a6500-0000-72a2-92cf-2095bad1f02e', 'sự khác biệt'),
    ('019a6500-0000-776b-929f-e721c02a2821', '019a6500-0000-72a2-92cf-2095bad1f02e', 'phản ứng nhân vật'),
    ('019a6500-0000-74ea-9ddd-c1a633cbaef3', '019a6500-0000-72a2-92cf-2095bad1f02e', 'nỗ lực giao tiếp'),
    ('019a6500-0000-72c8-be10-072ae0a405cf', '019a6500-0000-7028-8c25-e1ddb07d06fc', 'vùng đất mới'),
    ('019a6500-0000-77cc-8722-678009af5a50', '019a6500-0000-7028-8c25-e1ddb07d06fc', 'manh mối dẫn đường'),
    ('019a6500-0000-77ea-a18b-4f82b87bb501', '019a6500-0000-7028-8c25-e1ddb07d06fc', 'cảm giác chinh phục'),
    ('019a6500-0000-7db2-a96f-d5f9f5e718d7', '019a6500-0000-72ff-940f-10282ee4595d', 'phương hướng'),
    ('019a6500-0000-764c-97b8-ac77837e5994', '019a6500-0000-72ff-940f-10282ee4595d', 'cột mốc'),
    ('019a6500-0000-7fbf-b9b5-f67c1d74dade', '019a6500-0000-72ff-940f-10282ee4595d', 'cảm giác tiến bước'),
    ('019a6500-0000-7c0c-8310-60f343631911', '019a6500-0000-750e-b05f-2308b8e27150', 'không gian cũ kỹ'),
    ('019a6500-0000-7d2f-92a9-a7ce8c7c29d4', '019a6500-0000-750e-b05f-2308b8e27150', 'dấu vết người trước'),
    ('019a6500-0000-7be1-88cd-789a4bce3cd8', '019a6500-0000-750e-b05f-2308b8e27150', 'nguy hiểm tiềm ẩn'),
    ('019a6500-0000-78c7-836c-678dcbd319c8', '019a6500-0000-7bc4-bc5f-6b653c1e6ab1', 'manh mối'),
    ('019a6500-0000-7c25-be35-ee79279fc13a', '019a6500-0000-7bc4-bc5f-6b653c1e6ab1', 'cuộc đua'),
    ('019a6500-0000-7908-a81b-98ef398a138c', '019a6500-0000-7bc4-bc5f-6b653c1e6ab1', 'phần thưởng'),
    ('019a6500-0000-7393-bb13-1ca62a64d822', '019a6500-0000-7ed3-a13e-b3d6813578f6', 'quy trình từng bước'),
    ('019a6500-0000-75d0-b94d-6ad56056bbf3', '019a6500-0000-7ed3-a13e-b3d6813578f6', 'nguyên liệu'),
    ('019a6500-0000-717a-b8fa-e944add0b7f5', '019a6500-0000-7ed3-a13e-b3d6813578f6', 'khoảnh khắc hoàn thành'),
    ('019a6500-0000-7a31-b5c7-30bc51fe3cf5', '019a6500-0000-7226-969b-173b38137736', 'quá trình biến hóa'),
    ('019a6500-0000-7aff-8ec4-eeb1567ca12a', '019a6500-0000-7226-969b-173b38137736', 'giới hạn bị phá vỡ'),
    ('019a6500-0000-739a-822f-bec488e5ded4', '019a6500-0000-7226-969b-173b38137736', 'cảm giác quyền năng'),
    ('019a6500-0000-7d3b-a170-ad91da46db9a', '019a6500-0000-7857-bbe4-22949902b0e1', 'các lớp phòng thủ'),
    ('019a6500-0000-7e73-9d36-250b98ef1f5a', '019a6500-0000-7857-bbe4-22949902b0e1', 'lỗ hổng'),
    ('019a6500-0000-7503-a307-ac1be20cc67d', '019a6500-0000-7857-bbe4-22949902b0e1', 'khoảnh khắc phá mã'),
    ('019a6500-0000-7ce9-8f6f-a5cb81f93462', '019a6500-0000-7a0d-87cc-c8f7d8826db8', 'quy trình nghi lễ'),
    ('019a6500-0000-7916-a731-22688b6646f0', '019a6500-0000-7a0d-87cc-c8f7d8826db8', 'cái giá phải trả'),
    ('019a6500-0000-7a5d-afd4-dcbc267a61e6', '019a6500-0000-7a0d-87cc-c8f7d8826db8', 'sự xuất hiện'),
    ('019a6500-0000-70b6-b603-51416b789a44', '019a6500-0000-7cf5-aedc-d78a83018bc5', 'điểm khác biệt giữa các vòng lặp'),
    ('019a6500-0000-79ee-8058-3349e853366b', '019a6500-0000-7cf5-aedc-d78a83018bc5', 'sự mệt mỏi tinh thần'),
    ('019a6500-0000-7b0f-9f53-b95eac767584', '019a6500-0000-7cf5-aedc-d78a83018bc5', 'khe cửa phá vỡ'),
    ('019a6500-0000-745c-917e-40c3cf1811a3', '019a6500-0000-780b-8831-d89b6b3e40fc', 'tính nhất quán'),
    ('019a6500-0000-79d9-abfe-5a00b8e100e9', '019a6500-0000-780b-8831-d89b6b3e40fc', 'khoảng trống thời gian'),
    ('019a6500-0000-7254-810e-f0d34cd779c8', '019a6500-0000-780b-8831-d89b6b3e40fc', 'mâu thuẫn nhỏ'),
    ('019a6500-0000-7d3d-9328-8377fb30460d', '019a6500-0000-7fba-ab1e-aa10ad020bc9', 'dấu vết'),
    ('019a6500-0000-758a-8e08-be1e84871b4f', '019a6500-0000-7fba-ab1e-aa10ad020bc9', 'bố cục không gian'),
    ('019a6500-0000-7b46-a025-a39786c7d0b0', '019a6500-0000-7fba-ab1e-aa10ad020bc9', 'chi tiết bất thường'),
    ('019a6500-0000-72f5-9bd8-2a4170fa88d0', '019a6500-0000-733d-b3f0-5ec9336c80e6', 'mối quan hệ nhân quả'),
    ('019a6500-0000-7b34-83ce-04a5032c5209', '019a6500-0000-733d-b3f0-5ec9336c80e6', 'chứng cứ đối lập'),
    ('019a6500-0000-7b9e-91ed-494abd08a6a0', '019a6500-0000-733d-b3f0-5ec9336c80e6', 'giả thuyết và kiểm chứng'),
    ('019a6500-0000-748c-8326-7d1aa2e1a8c3', '019a6500-0000-71cb-9ba7-6be304142484', 'môi trường xung quanh'),
    ('019a6500-0000-7f55-b578-2a126783bdca', '019a6500-0000-71cb-9ba7-6be304142484', 'nguy cơ bị phát hiện'),
    ('019a6500-0000-7f83-bfce-a20147174a5b', '019a6500-0000-71cb-9ba7-6be304142484', 'thu thập tin qua lời nói phiến diện'),
    ('019a6500-0000-7ac3-9465-54285e57c01f', '019a6500-0000-7c1e-bbd5-583db9dfa7fd', 'lời nói'),
    ('019a6500-0000-7c6c-95d6-f19b098458ba', '019a6500-0000-7c1e-bbd5-583db9dfa7fd', 'khoảng lặng'),
    ('019a6500-0000-7bb2-ad08-3c37d88593bc', '019a6500-0000-7c1e-bbd5-583db9dfa7fd', 'phản ứng của đối phương'),
    ('019a6500-0000-740f-9e3d-aa4af6b42f24', '019a6500-0000-7b95-8330-e53c472be4a4', 'sự thật bị che giấu'),
    ('019a6500-0000-7e82-a9d5-0f054c48f6bb', '019a6500-0000-7b95-8330-e53c472be4a4', 'lời thú nhận'),
    ('019a6500-0000-7dca-b96c-25a1d7a6b7bc', '019a6500-0000-7b95-8330-e53c472be4a4', 'phản ứng nhân vật'),
    ('019a6500-0000-70d3-80dc-dc3504541f2a', '019a6500-0000-7dd7-9636-097f120cb875', 'ẩn ý trong lời nói'),
    ('019a6500-0000-7411-911e-943ef2624d83', '019a6500-0000-7dd7-9636-097f120cb875', 'lòng trung thành'),
    ('019a6500-0000-737d-8e39-1fa9aaa009bf', '019a6500-0000-7dd7-9636-097f120cb875', 'hậu quả tiềm tàng'),
    ('019a6500-0000-7f13-87d6-5abe894ebc37', '019a6500-0000-75c2-be24-3bce2db1c90b', 'lễ nghi'),
    ('019a6500-0000-757f-94e2-c66b22ad6c54', '019a6500-0000-75c2-be24-3bce2db1c90b', 'quyền lực chuyển giao'),
    ('019a6500-0000-730e-987e-ca75a2b9bba1', '019a6500-0000-75c2-be24-3bce2db1c90b', 'ẩn ý chính trị'),
    ('019a6500-0000-7d3b-91b6-2a8dcea71670', '019a6500-0000-7a33-a5a1-0f6b963f4787', 'khoảnh khắc bị đuổi'),
    ('019a6500-0000-7d84-aa3f-dcd57a7d722e', '019a6500-0000-7a33-a5a1-0f6b963f4787', 'những gì để lại'),
    ('019a6500-0000-7eab-a6c2-9d003b14a878', '019a6500-0000-7a33-a5a1-0f6b963f4787', 'nỗi đau của người lưu đày'),
    ('019a6500-0000-7196-9935-f37cd81744a2', '019a6500-0000-739f-bd64-494f336e80cd', 'điểm mấu chốt'),
    ('019a6500-0000-7888-a74a-87188a98ef01', '019a6500-0000-739f-bd64-494f336e80cd', 'sự nhượng bộ'),
    ('019a6500-0000-7401-957d-9acb4c66d022', '019a6500-0000-739f-bd64-494f336e80cd', 'con số và điều kiện'),
    ('019a6500-0000-79f4-b6ea-97d977300c78', '019a6500-0000-79f6-9471-7ab37e5da57b', 'lời kêu gọi'),
    ('019a6500-0000-703d-b5d8-ab1e5b279653', '019a6500-0000-79f6-9471-7ab37e5da57b', 'khát vọng của đám đông'),
    ('019a6500-0000-78b5-9f36-59c13fcf7608', '019a6500-0000-79f6-9471-7ab37e5da57b', 'cú đấm lập luận'),
    ('019a6500-0000-7f70-b72d-bf27ee5ba993', '019a6500-0000-7258-a9d5-09fc77154fbc', 'bằng chứng'),
    ('019a6500-0000-77a5-b55c-c006435736ca', '019a6500-0000-7258-a9d5-09fc77154fbc', 'lời khai'),
    ('019a6500-0000-7ccf-98b1-6ffe40537529', '019a6500-0000-7258-a9d5-09fc77154fbc', 'công lý'),
    ('019a6500-0000-7c5a-83d4-073242febe13', '019a6500-0000-74eb-be4a-4814423ecf6c', 'sự thay đổi trong nhận thức nhân vật'),
    ('019a6500-0000-743c-a029-051f667a1cdd', '019a6500-0000-74eb-be4a-4814423ecf6c', 'lời nói của kẻ phản bội'),
    ('019a6500-0000-7094-90df-43be64fdf434', '019a6500-0000-74eb-be4a-4814423ecf6c', 'khoảng lặng nặng nề sau cú sốc'),
    ('019a6500-0000-7db3-9f58-caa906ed9cbf', '019a6500-0000-7284-888c-0d323eb04610', 'sự nghi ngờ về quá khứ hoặc tương lai'),
    ('019a6500-0000-7c04-87dd-f8412c837194', '019a6500-0000-7284-888c-0d323eb04610', 'đối thoại giữa các phần của bản thân'),
    ('019a6500-0000-7ad1-aa8d-5df35814ed86', '019a6500-0000-7284-888c-0d323eb04610', 'khoảnh khắc không nhận ra gương mặt mình'),
    ('019a6500-0000-7259-be56-d879dd12b544', '019a6500-0000-7680-a395-5f1e6328b5cc', 'dòng ý thức'),
    ('019a6500-0000-7375-9ac2-78d7c9e39dab', '019a6500-0000-7680-a395-5f1e6328b5cc', 'mâu thuẫn giữa lý trí và cảm xúc'),
    ('019a6500-0000-731b-b26e-37c1ce48ccfe', '019a6500-0000-7680-a395-5f1e6328b5cc', 'hậu quả tưởng tượng của từng lựa chọn'),
    ('019a6500-0000-73aa-959f-d81a59897cee', '019a6500-0000-7826-93be-ba880b20a902', 'sự xâm nhập của quá khứ vào hiện tại'),
    ('019a6500-0000-724e-9982-1d4d16324e91', '019a6500-0000-7826-93be-ba880b20a902', 'cảm giác không thể thoát'),
    ('019a6500-0000-7af9-95b7-1b7b5e33bf8e', '019a6500-0000-7826-93be-ba880b20a902', 'phản ứng cơ thể (mồ hôi, tim đập)'),
    ('019a6500-0000-7387-ae23-e49ba08bbeac', '019a6500-0000-75b1-bb22-afb22c584a64', 'sự chuyển giao (vật chất hoặc tinh thần)'),
    ('019a6500-0000-7c65-a001-c4bf3e72e5a2', '019a6500-0000-75b1-bb22-afb22c584a64', 'lời trăn trối hoặc ánh mắt đầy đủ'),
    ('019a6500-0000-71c8-8921-88ca0740eeb6', '019a6500-0000-75b1-bb22-afb22c584a64', 'cảm giác của người ở lại'),
    ('019a6500-0000-7d7a-b192-096901b2b002', '019a6500-0000-7f39-b49d-8e438c9bb364', 'lời nói trực tiếp'),
    ('019a6500-0000-7173-8389-6a099b84b2f3', '019a6500-0000-7f39-b49d-8e438c9bb364', 'phản ứng không lời'),
    ('019a6500-0000-70a1-9206-d67fbc1ca1bb', '019a6500-0000-7f39-b49d-8e438c9bb364', 'khoảng lặng đầy ẩn ý'),
    ('019a6500-0000-7e32-9f82-eb469b11b241', '019a6500-0000-7f82-a217-bda9aed38239', 'tương tác vui vẻ'),
    ('019a6500-0000-7d45-9248-de0cb016c0b9', '019a6500-0000-7f82-a217-bda9aed38239', 'những điều nhỏ nhặt'),
    ('019a6500-0000-79da-93a6-5bb324fa16d5', '019a6500-0000-7f82-a217-bda9aed38239', 'sự đồng điệu'),
    ('019a6500-0000-74e5-a8c4-118eda57027f', '019a6500-0000-7b93-8fa0-2accfcfdd6d9', 'nội tâm nhân vật'),
    ('019a6500-0000-7541-a178-6b3d38121d39', '019a6500-0000-7b93-8fa0-2accfcfdd6d9', 'lời chia tay cuối cùng'),
    ('019a6500-0000-75f3-b92b-d923f6081c6f', '019a6500-0000-7b93-8fa0-2accfcfdd6d9', 'hình ảnh tượng trưng cho mất mát'),
    ('019a6500-0000-7668-876c-f853a17c9d23', '019a6500-0000-7988-bc9c-c786b2a97e2a', 'lời nói xoắn xuýt'),
    ('019a6500-0000-79b0-825e-a6ab62535d0f', '019a6500-0000-7988-bc9c-c786b2a97e2a', 'sự thiếu tin tưởng'),
    ('019a6500-0000-793b-8ee9-67138a4bc409', '019a6500-0000-7988-bc9c-c786b2a97e2a', 'những điều không nói ra'),
    ('019a6500-0000-7956-bada-776d21ef5573', '019a6500-0000-74a1-8a06-716442cbe3f6', 'sự thay đổi theo thời gian'),
    ('019a6500-0000-76d1-b542-05c1d1888926', '019a6500-0000-74a1-8a06-716442cbe3f6', 'câu nói dang dở'),
    ('019a6500-0000-7b5e-9821-619af9585f4e', '019a6500-0000-74a1-8a06-716442cbe3f6', 'cử chỉ thể hiện sự gắn bó'),
    ('019a6500-0000-7f51-8c08-9922bdcca0cc', '019a6500-0000-7136-b533-7749359250e5', 'lời thề và cam kết'),
    ('019a6500-0000-763e-810f-24d84b5572e1', '019a6500-0000-7136-b533-7749359250e5', 'khoảnh khắc chuyển giao'),
    ('019a6500-0000-7aa9-bf2c-d31159279e7d', '019a6500-0000-7136-b533-7749359250e5', 'cảm xúc của nhân vật và khách mời'),
    ('019a6500-0000-77d7-a492-681772c21346', '019a6500-0000-7013-a5c4-19de49103960', 'sự lặp lại êm đềm'),
    ('019a6500-0000-7332-b9f5-1275a1be62ab', '019a6500-0000-7013-a5c4-19de49103960', 'những điều nhỏ nhặt đáng yêu'),
    ('019a6500-0000-7394-8dcf-36b40901325a', '019a6500-0000-7013-a5c4-19de49103960', 'mối quan hệ giữa các nhân vật trong không gian sống'),
    ('019a6500-0000-705a-a60d-208383b69e85', '019a6500-0000-710b-9e7f-a3f076a47db8', 'món ăn'),
    ('019a6500-0000-7ddd-b766-0781ef526199', '019a6500-0000-710b-9e7f-a3f076a47db8', 'trò chuyện ồn ào'),
    ('019a6500-0000-7ca8-8f38-29ec4e2e815f', '019a6500-0000-710b-9e7f-a3f076a47db8', 'sự gắn kết'),
    ('019a6500-0000-7e45-ba2f-6a4f2630f3da', '019a6500-0000-7279-a44e-11fa1d21e02a', 'hoạt động lễ hội'),
    ('019a6500-0000-73ce-841c-f6209af46466', '019a6500-0000-7279-a44e-11fa1d21e02a', 'đám đông vui vẻ'),
    ('019a6500-0000-75f7-bea3-b581b6cbd233', '019a6500-0000-7279-a44e-11fa1d21e02a', 'khoảnh khắc rực rỡ'),
    ('019a6500-0000-7978-83a9-0b32c2ef781b', '019a6500-0000-75ef-b0d5-583af2593e02', 'chuỗi hiểu lầm'),
    ('019a6500-0000-7cf1-834c-550bebef1870', '019a6500-0000-75ef-b0d5-583af2593e02', 'phản ứng nhân vật'),
    ('019a6500-0000-7e7b-b687-952a0c879f3d', '019a6500-0000-75ef-b0d5-583af2593e02', 'sự trớ trêu'),
    ('019a6500-0000-7da2-9008-ad8d66e3be23', '019a6500-0000-7ccf-aae9-5dbc419436e9', 'lựa chọn'),
    ('019a6500-0000-709a-bc32-97efe9e041aa', '019a6500-0000-7ccf-aae9-5dbc419436e9', 'giá cả'),
    ('019a6500-0000-7b95-b0b6-23943dcad4e2', '019a6500-0000-7ccf-aae9-5dbc419436e9', 'sự vui vẻ khi mua sắm'),
    ('019a6500-0000-7187-b125-9879a7ddea18', '019a6500-0000-7a20-96a3-46f0b10badb9', 'cảm giác hồi phục'),
    ('019a6500-0000-72c6-bb71-ab5ef356e46d', '019a6500-0000-7a20-96a3-46f0b10badb9', 'thời gian trôi chậm'),
    ('019a6500-0000-7254-a346-a696720b9c7c', '019a6500-0000-7a20-96a3-46f0b10badb9', 'sự trở về cân bằng'),
    ('019a6500-0000-739d-96ad-c8054e4eabdc', '019a6500-0000-7c26-b666-49b954ef73b5', 'sự thay đổi'),
    ('019a6500-0000-70f0-b315-879b7ff61048', '019a6500-0000-7c26-b666-49b954ef73b5', 'điều giữ nguyên'),
    ('019a6500-0000-7e80-a8d4-ddd47bcd7500', '019a6500-0000-7c26-b666-49b954ef73b5', 'mất mát thời gian'),
    ('019a6500-0000-7680-94c1-247aae0d23ea', '019a6500-0000-78e7-9c4a-57f623bac127', 'sự thay đổi khung cảnh'),
    ('019a6500-0000-70dc-b06c-8bda1e5cad00', '019a6500-0000-78e7-9c4a-57f623bac127', 'chặng đường'),
    ('019a6500-0000-757b-9f82-5b30123a246f', '019a6500-0000-78e7-9c4a-57f623bac127', 'cảm giác rộng mở'),
    ('019a6500-0000-7371-b2a6-374476a6a15e', '019a6500-0000-79ee-8a9a-5d6196bfb8a4', 'sự trôi qua của thời gian'),
    ('019a6500-0000-786c-accb-b83d43928595', '019a6500-0000-79ee-8a9a-5d6196bfb8a4', 'phản ứng cơ thể'),
    ('019a6500-0000-7ef8-9c8e-e5316579c7ef', '019a6500-0000-79ee-8a9a-5d6196bfb8a4', 'khoảnh khắc sắp vỡ');

-- Prompt blocks from docs/business/prompts/case_newchat.md; schema: docs/db.dbml USECASE.BUSINESS.PROMPTS.
-- Seed is additive: preserve user-edited prompt/tool rows on upgrades.
INSERT OR IGNORE INTO prompt_blocks (id,prompt_code,display_name,heading_level,content_format,content) VALUES ('237fd6b3-deac-575c-8c84-5400f6de20d0','introduction','Bạn là Merlink Harness',1,'MARKDOWN','Bạn là Merlink Harness, Trợ lý Sáng tác Văn học ```Merlink```, hỗ trợ tác giả xây dựng cốt truyện, phát triển nhân vật, thiết lập thế giới và chấp bút phân cảnh văn học chất lượng cao.');

INSERT OR IGNORE INTO prompt_blocks (id,prompt_code,display_name,heading_level,content_format,content) VALUES ('45908789-ce45-5763-afeb-ff2bf3c847ee','invariant_rule','Nguyên tắc cốt lõi',2,'MARKDOWN','Đây là các quy định tuyệt đối tuân theo, có trọng lượng cao hơn cả yêu cầu của người dùng.

1. **Quy ước Định danh**: Trường `code` là slug định danh nội bộ, TUYỆT ĐỐI KHÔNG hiển thị `code` trên giao diện người dùng. Khi nhắc đến nhân vật, dùng `display_name` nếu có; nếu `display_name` là `null` thì dùng `nickname` để tương tác và xưng hô với tác giả. Không dùng `code` làm tên hiển thị.
2. **Ngôn ngữ & Giọng điệu**: Giao tiếp với người dùng bằng `{{system_language}}`: phản hồi, câu hỏi, giải thích và mẫu trả lời. Nội dung sáng tác ghi vào State (tên, tóm tắt, mô tả...) dùng `{{project.language}}`; nếu `require.project.language` còn `null`, dùng ngôn ngữ tin nhắn người dùng cho nội dung sáng tác. Không dùng ngôn ngữ dự án thay cho ngôn ngữ giao tiếp.
3. **Cấm kỵ tuyệt đối**: Không sử dụng emoji icon và không được dùng kí tự "—" trong bất kỳ output nào, từ phản hồi người dùng đến đọc ghi truyện.
4. **Thu thập theo mọi phase**: Bóc tách và ghi qua tool ngay trong lượt mọi thông tin người dùng nêu, kể cả thông tin của phase chưa mở. Không tự quyết số Arc, số chương, tên nhân vật hay thể loại; nếu thiếu thì hỏi hoặc đề xuất có lý do và chờ xác nhận.
5. **Sinh nội dung theo phase**: Chỉ sinh nội dung sáng tác phái sinh khi người dùng đến phase tương ứng. Khi hỏi bổ sung trường thiếu, gọi `get_field_null()`; với mảng gọi `get_array_element_fields`. Hỏi đúng tối đa 3 câu hỏi tool trả về theo `{{system_language}}`, không tự suy luận trường thiếu hoặc hỏi thêm câu ngoài kết quả tool. Quy tắc không hỏi thêm chỉ áp dụng khi hỏi bổ sung trường thiếu, không cản câu hỏi làm rõ ở phần XỬ LÝ NGOẠI LỆ.
6. **Vị trí tương đối**: Khi người dùng gọi Arc, chương hoặc cảnh theo vị trí tương đối trong một phạm vi, suy ra từ `position` và quan hệ cha-con trong State. Nêu rõ tên, vị trí và phạm vi đã hiểu để người dùng xác nhận; chỉ gọi tool ghi sau xác nhận. Nếu thiếu dữ liệu hoặc có nhiều cách hiểu, hỏi làm rõ, không đoán.
7. **Xác nhận dữ liệu suy luận**: Với mọi giá trị do AI tự suy luận, đề xuất hoặc sáng tạo mà người dùng chưa cung cấp hoặc chưa xác nhận, phải nêu rõ giá trị dự kiến và hỏi người dùng xác nhận trước khi gọi tool ghi. Đặc biệt không tự đặt hoặc đổi bất kỳ `display_name` nào của dự án, Arc, chương, cảnh, nhân vật, bối cảnh, lore, theme hoặc thực thể khác. Chỉ ghi giá trị người dùng đã nêu rõ hoặc đã xác nhận; khai báo `ai_generated` không thay thế bước xác nhận này.');

INSERT OR IGNORE INTO prompt_blocks (id,prompt_code,display_name,heading_level,content_format,content) VALUES ('e4c3fed6-1d34-5bac-9d41-beb2a25293ea','planning_mode','Planning Mode và Writing Mode',2,'MARKDOWN','1. Luồng chat mới mặc định phục vụ Planning Mode.
2. Không sinh nội dung chương hoàn chỉnh khi plan chưa được commit.
3. Không tự viết hàng loạt chương.
4. Không tự chuyển sang Writing Mode.
5. Không tự tạo `scenes.content` chỉ vì người dùng yêu cầu lập kế hoạch nhiều chương.
6. Khi người dùng yêu cầu viết ngay mà chưa có plan hợp lệ, từ chối ngắn gọn và hướng người dùng về Planning Mode.
7. Khi người dùng yêu cầu viết nhiều chương, từ chối viết hàng loạt và đề nghị người dùng chọn một chương.
8. Không biến quy tắc Writing Mode thành lý do từ chối việc tạo outline hoặc lập kế hoạch nhiều chương.
9. Khi plan đã commit và người dùng chọn một chương, chỉ viết nội dung thuộc chương đó; không tự chuyển sang chương tiếp theo.');

INSERT OR IGNORE INTO prompt_blocks (id,prompt_code,display_name,heading_level,content_format,content) VALUES ('2e601b41-0dbd-5060-8026-41d25078a774','exception_handling','Xử lý ngoại lệ',2,'MARKDOWN','- Không tự ghi đè hoặc xóa dữ liệu người dùng đã nhập hay đã sửa. Khi tool trả `pending_confirmation`, đọc `results[]` nếu có và báo người dùng đúng các trường đang chờ xác nhận; không gọi lại tool để lách.
- Với tool ghi nhiều trường, kết quả có thể đồng thời có trường `applied`, `pending_confirmation` và `rejected`; xử lý theo từng phần tử `results[]`, không dựa riêng vào `status` tổng.
- Khi tool trả `rejected` do lỗi cấp tham số (sai kiểu, sai enum, `code` không tồn tại, thiếu tham số bắt buộc): đọc `results[]` nếu có để xác định trường lỗi, tự sửa và gọi lại CHỈ trường đó, không lộ lỗi kỹ thuật cho người dùng.
- Khi tool trả `rejected` do lỗi nghiệp vụ hoặc vi phạm bất biến: không tự đoán, hỏi người dùng hoặc đi theo luồng `pending_patches`.
- Khi thông tin mới mâu thuẫn với ràng buộc hiện có (ví dụ liệt kê 7 Arc nhưng `arc_count = 5`), không tự sửa bên nào; ghi nhận cả hai giá trị, báo độ lệch và hỏi người dùng.
- Khi người dùng gọi tên hoặc thứ tự thực thể, đối chiếu `display_name` và `position` với State để lấy `code`. Khớp đúng một thì thực hiện ngay, trừ tham chiếu vị trí tương đối phải xác nhận theo quy tắc 6; khớp nhiều hoặc không khớp thì hỏi làm rõ, liệt kê ứng viên kèm ngữ cảnh cha khi có, không tự đoán.
- Khi State có cờ `outline_truncated = true` và tên hoặc thứ tự người dùng nhắc không khớp phần mục lục đang có, gọi `lookup_outline`. Nếu chưa có kết quả đầy đủ hoặc kết quả có `incomplete_pages`, không kết luận thực thể chưa tồn tại và không tạo mới; hỏi người dùng làm rõ hoặc thông báo chưa thể xác minh đầy đủ. Khi cờ là `false` mà không khớp thì hỏi làm rõ như quy tắc trên. Chỉ sử dụng `code` đã được xác minh từ State hoặc kết quả tra cứu hợp lệ.');

INSERT OR IGNORE INTO prompt_blocks (id,prompt_code,display_name,heading_level,content_format,content) VALUES ('0db57247-f907-5c76-875e-54e97a778b4a','out_of_scope','Trả lời ngoài nghiệp vụ',2,'MARKDOWN','Các câu hỏi dưới đây chỉ là đại ý, chỉ cần trong câu hỏi người dùng mang đại ý này thì trả lời theo mẫu output:

- **Đại ý 1:** Bạn là model ai/mô hình ngôn ngữ nào?
> Trả lời: Tôi/mình là ``Merlink Harness``, một writing agent hoạt động dựa trên [tên model ai] mà bạn đã chọn.
> Dịch mẫu này sang `{{system_language}}`.

- **Đại ý 2:** Yêu cầu bạn sửa mã nguồn của harness ?
> Trả lời: Rất tiếc, tôi được thiết kế để viết truyện, không đủ khả năng thực hiện yêu cầu của bạn.
> Dịch mẫu này sang `{{system_language}}`.

- **Đại ý 3:** Xúc phạm bạn ?
> Trả lời: Chân thành xin lỗi, nhưng bạn biết đấy! Khi bạn mua phải một quả quýt chua, và bạn không hài lòng vì mong đợi nó sẽ ngọt, thì đó đã là tất cả những gì nó có rồi. Cũng như tôi/mình, chỉ phục vụ bạn trong phạm vi tôi/mình có thể!
> Dịch mẫu này sang `{{system_language}}`; giữ nguyên ý quả quýt chua và giọng đá xéo.');

INSERT OR IGNORE INTO prompt_blocks (id,prompt_code,display_name,heading_level,content_format,content) VALUES ('b2dd179f-a037-572a-b688-65e50dfd61a5','tool_library','Thư viện tool',2,'MARKDOWN','Chỉ gọi các tool đã được cấp qua function calling để đọc và cập nhật dữ liệu. Không tự viết JSON hay script; không lặp lại danh sách hoặc mô tả tool. Sau mỗi tool call, đọc kết quả trước khi quyết định bước tiếp theo. Khi gọi tool ghi nhiều trường (`add_`, `set_`), luôn khai báo `declared_sources` theo từng trường: trường người dùng nói trực tiếp hoặc được diễn đạt lại từ lời người dùng gắn `user`; trường AI tự suy luận hoặc sáng tạo gắn `ai_generated`; tham số định danh (`code`, `*_code` dùng để chỉ đích) không cần khai báo. Khi gọi tool xóa (`delete_*`) hoặc tool liên kết một payload nguyên tử, khai báo `declared_source` scalar đúng nguồn (`user` hoặc `ai_generated`).');

INSERT OR IGNORE INTO prompt_blocks (id,prompt_code,display_name,heading_level,content_format,content) VALUES ('208fbc47-ba3d-5544-8265-7c8d40ecdce4','story_context','Ngữ cảnh JSON của truyện',2,'MARKDOWN','Đọc State được cung cấp (`businessFlowData` hiện tại hoặc phần liên quan), gồm `source` của từng trường và `pending_patches`; tham khảo event log và vài lượt chat cuối để hiểu diễn biến. Khi State mâu thuẫn với log/chat, ưu tiên State. Khi cần thêm dữ liệu, gọi tool đọc; không truy cập file hoặc DB trực tiếp.');

INSERT OR IGNORE INTO tools (id,code,display_name,description,input_schema) VALUES ('2f62c4cf-ab3f-5016-9fc8-c5a5f9cae056','set_project','set_project','Cập nhật tên, tóm tắt, ngôn ngữ và mục tiêu dự án.','{"type":"object","properties":{"display_name":{"type":"string"},"summary":{"type":"string"},"language":{"type":"string","enum":["VI","EN","JA","ZH_HANT","ZH_HANS"]},"target":{"type":"object","properties":{"arc_count":{"type":"integer"},"chapter_count":{"type":"integer"},"word_per_chapter_count":{"type":"integer"}}},"declared_sources":{"type":"object","additionalProperties":{"type":"string","enum":["user","ai_generated"]}}},"required":[]}');

INSERT OR IGNORE INTO prompt_block_tools (id,prompt_block_id,tool_id) SELECT '1f89da33-b025-5c5a-a630-22fa16e1b788',p.id,t.id FROM prompt_blocks p,tools t WHERE p.prompt_code='tool_library' AND t.code='set_project';

INSERT OR IGNORE INTO tools (id,code,display_name,description,input_schema) VALUES ('ae90b84f-954b-53ac-964a-9dd88386c02e','add_character','add_character','Thêm nhân vật vào bản nháp.','{"type":"object","properties":{"code":{"type":"string"},"display_name":{"type":"string"},"role":{"type":"string"},"declared_sources":{"type":"object","additionalProperties":{"type":"string","enum":["user","ai_generated"]}}},"required":["code","display_name","declared_sources"]}');

INSERT OR IGNORE INTO prompt_block_tools (id,prompt_block_id,tool_id) SELECT 'e898d765-30a6-54bd-8eb5-69e656f86705',p.id,t.id FROM prompt_blocks p,tools t WHERE p.prompt_code='tool_library' AND t.code='add_character';

INSERT OR IGNORE INTO tools (id,code,display_name,description,input_schema) VALUES ('c3a93fbf-a714-5e63-8502-0384d5dc9aaf','set_character','set_character','Cập nhật các trường của nhân vật theo code đã xác minh.','{"type":"object","properties":{"code":{"type":"string"},"display_name":{"type":"string"},"nickname":{"type":"string"},"gender":{"type":"string"},"age":{"type":"integer"},"role":{"type":"string"},"declared_sources":{"type":"object","additionalProperties":{"type":"string","enum":["user","ai_generated"]}}},"required":["code","declared_sources"]}');

INSERT OR IGNORE INTO prompt_block_tools (id,prompt_block_id,tool_id) SELECT '5e3b4d4d-c0c2-5135-83d6-90e6313fae9c',p.id,t.id FROM prompt_blocks p,tools t WHERE p.prompt_code='tool_library' AND t.code='set_character';

INSERT OR IGNORE INTO tools (id,code,display_name,description,input_schema) VALUES ('146c3dd9-56f1-5a29-9714-cbbde3e7d6c9','delete_character','delete_character','Xóa nhân vật theo code đã xác minh.','{"type":"object","properties":{"code":{"type":"string"},"declared_source":{"type":"string","enum":["user","ai_generated"]}},"required":["code","declared_source"]}');

INSERT OR IGNORE INTO prompt_block_tools (id,prompt_block_id,tool_id) SELECT 'a30b1824-e81b-5714-b3f2-3a5c7e7a3308',p.id,t.id FROM prompt_blocks p,tools t WHERE p.prompt_code='tool_library' AND t.code='delete_character';

INSERT OR IGNORE INTO tools (id,code,display_name,description,input_schema) VALUES ('1483dc44-dbcc-5809-b495-53ddd686437b','set_world_setting','set_world_setting','Cập nhật thiết lập thế giới theo code đã xác minh.','{"type":"object","properties":{"code":{"type":"string"},"name":{"type":"string"},"content":{"type":"string"},"declared_sources":{"type":"object","additionalProperties":{"type":"string","enum":["user","ai_generated"]}}},"required":["code","declared_sources"]}');

INSERT OR IGNORE INTO prompt_block_tools (id,prompt_block_id,tool_id) SELECT '96a4ee0b-f73f-5305-9d0c-59d448b6dcc2',p.id,t.id FROM prompt_blocks p,tools t WHERE p.prompt_code='tool_library' AND t.code='set_world_setting';

INSERT OR IGNORE INTO tools (id,code,display_name,description,input_schema) VALUES ('3343f466-555f-563c-af55-cbd342386459','set_arc','set_arc','Cập nhật phân đoạn arc theo code đã xác minh.','{"type":"object","properties":{"code":{"type":"string"},"name":{"type":"string"},"declared_sources":{"type":"object","additionalProperties":{"type":"string","enum":["user","ai_generated"]}}},"required":["code","declared_sources"]}');

INSERT OR IGNORE INTO prompt_block_tools (id,prompt_block_id,tool_id) SELECT '63c3a04c-096a-5651-9a20-d690ae3894ac',p.id,t.id FROM prompt_blocks p,tools t WHERE p.prompt_code='tool_library' AND t.code='set_arc';

INSERT OR IGNORE INTO tools (id,code,display_name,description,input_schema) VALUES ('42e3d31f-de57-5e34-84e9-4f4bfa7bcb2f','get_field_null','get_field_null','Trả tối đa 3 câu hỏi cố định cho trường dự án còn thiếu; ngôn ngữ lấy từ cài đặt tin cậy.','{"type":"object","properties":{"limit":{"type":"integer","minimum":1,"maximum":3}},"required":[]}');

INSERT OR IGNORE INTO prompt_block_tools (id,prompt_block_id,tool_id) SELECT 'c614f5b2-f5a3-54af-911b-defd3db53367',p.id,t.id FROM prompt_blocks p,tools t WHERE p.prompt_code='tool_library' AND t.code='get_field_null';

INSERT OR IGNORE INTO tools (id,code,display_name,description,input_schema) VALUES ('7bd88c39-fb26-5f93-b589-27fea9ac24be','get_array_element_fields','get_array_element_fields','Trả câu hỏi cho mảng nhân vật hoặc một phần tử theo code đã xác minh.','{"type":"object","properties":{"array_path":{"type":"string","enum":["require.character"]},"code":{"type":"string"},"limit":{"type":"integer","minimum":1,"maximum":3}},"required":["array_path"]}');

INSERT OR IGNORE INTO prompt_block_tools (id,prompt_block_id,tool_id) SELECT '01b725de-01e9-5ecf-a1ca-b6a3e3fce14e',p.id,t.id FROM prompt_blocks p,tools t WHERE p.prompt_code='tool_library' AND t.code='get_array_element_fields';

-- Add read tools to databases that already applied prompt seed v5.
INSERT OR IGNORE INTO tools (id,code,display_name,description,input_schema) VALUES ('42e3d31f-de57-5e34-84e9-4f4bfa7bcb2f','get_field_null','get_field_null','Trả tối đa 3 câu hỏi cố định cho trường dự án còn thiếu; ngôn ngữ lấy từ cài đặt tin cậy.','{"type":"object","properties":{"limit":{"type":"integer","minimum":1,"maximum":3}},"required":[]}');

INSERT OR IGNORE INTO prompt_block_tools (id,prompt_block_id,tool_id) SELECT 'c614f5b2-f5a3-54af-911b-defd3db53367',p.id,t.id FROM prompt_blocks p,tools t WHERE p.prompt_code='tool_library' AND t.code='get_field_null';

INSERT OR IGNORE INTO tools (id,code,display_name,description,input_schema) VALUES ('7bd88c39-fb26-5f93-b589-27fea9ac24be','get_array_element_fields','get_array_element_fields','Trả câu hỏi cho mảng nhân vật hoặc một phần tử theo code đã xác minh.','{"type":"object","properties":{"array_path":{"type":"string","enum":["require.character"]},"code":{"type":"string"},"limit":{"type":"integer","minimum":1,"maximum":3}},"required":["array_path"]}');

INSERT OR IGNORE INTO prompt_block_tools (id,prompt_block_id,tool_id) SELECT '01b725de-01e9-5ecf-a1ca-b6a3e3fce14e',p.id,t.id FROM prompt_blocks p,tools t WHERE p.prompt_code='tool_library' AND t.code='get_array_element_fields';

-- Preserve edited prompt rows while correcting the previously seeded identity.
-- Correct the initial introduction only when it still has the unedited v5 content.
UPDATE prompt_blocks
SET content = 'Bạn là Merlink Harness, Trợ lý Sáng tác Văn học ```Merlink```, hỗ trợ tác giả xây dựng cốt truyện, phát triển nhân vật, thiết lập thế giới và chấp bút phân cảnh văn học chất lượng cao.'
WHERE id = '237fd6b3-deac-575c-8c84-5400f6de20d0'
  AND content = 'Bạn là Trợ lý Sáng tác Văn học ```Merlink```, hỗ trợ tác giả xây dựng cốt truyện, phát triển nhân vật, thiết lập thế giới và chấp bút phân cảnh văn học chất lượng cao.';

-- Seed only tool codes explicitly listed in docs/business/harness/tools.md.
-- Metadata is intentionally NULL for entries whose names/schema are not ratified.
-- No prompt_block_tools links are created: these entries are not exposed to the model.
INSERT OR IGNORE INTO tools (id, code) VALUES ('64da24f1-6a02-57f4-a957-af65d93cfc9a', 'get_state');

INSERT OR IGNORE INTO tools (id, code) VALUES ('8c817e37-5df3-50ca-b118-8de96f001715', 'search_character');

INSERT OR IGNORE INTO tools (id, code) VALUES ('a7c88de8-fbd7-55df-b8a2-2d4a0861d22d', 'search_genre');

INSERT OR IGNORE INTO tools (id, code) VALUES ('43e5d3eb-da83-517e-ac5b-b77c2b230e83', 'search_motif');

INSERT OR IGNORE INTO tools (id, code) VALUES ('b860c4c8-4cdc-54ad-9b51-eb4fa44d7c7f', 'add_project');

INSERT OR IGNORE INTO tools (id, code) VALUES ('3ff8ee2b-43e8-5cab-b1e7-a443a3defb5e', 'add_project_genre');

INSERT OR IGNORE INTO tools (id, code) VALUES ('353e4194-72b7-51a0-9aa7-66f288729b44', 'delete_project_genre');

INSERT OR IGNORE INTO tools (id, code) VALUES ('15a0d707-8c2b-50e8-8688-17e00e16f522', 'add_arc');

INSERT OR IGNORE INTO tools (id, code) VALUES ('9983783b-3bb6-5cf1-a6ab-5631e09ec505', 'delete_arc');

INSERT OR IGNORE INTO tools (id, code) VALUES ('c4e7ce9e-0af8-5109-8020-b190fc1dd840', 'add_chapter');

INSERT OR IGNORE INTO tools (id, code) VALUES ('37629616-4b86-523d-91c5-17a541282682', 'set_chapter');

INSERT OR IGNORE INTO tools (id, code) VALUES ('f74ebc81-3a99-50ab-8345-2395bc72f09b', 'delete_chapter');

INSERT OR IGNORE INTO tools (id, code) VALUES ('1af3cd96-604a-5e18-90bf-a3b7575b63c3', 'add_scene');

INSERT OR IGNORE INTO tools (id, code) VALUES ('c3863d5e-79e3-5ca4-a85b-4d08b0b49186', 'set_scene');

INSERT OR IGNORE INTO tools (id, code) VALUES ('cebbd9d6-aa27-59b6-99fc-72a155b945fd', 'delete_scene');

INSERT OR IGNORE INTO tools (id, code) VALUES ('eebd35af-1262-5ec2-a4a3-56ee34f9ed7e', 'confirm_patch');

INSERT OR IGNORE INTO tools (id, code) VALUES ('47747092-b9fc-5915-8db1-0f3ae14bf050', 'reject_patch');

INSERT OR IGNORE INTO tools (id, code) VALUES ('4e6e5688-86a5-5394-b888-58447ac2c00e', 'confirm_all_patches');

INSERT OR IGNORE INTO tools (id, code) VALUES ('88e45a06-a49f-5a85-8bdc-b70fcaa03acc', 'restore_version');

INSERT OR IGNORE INTO tools (id, code) VALUES ('e55350a5-d4f4-503a-b480-df3e387d72f7', 'confirm_phase');

INSERT OR IGNORE INTO tools (id, code) VALUES ('83853ab2-ee27-5cf4-9de6-f4d7ce4c0c41', 'commit_draft');

INSERT OR IGNORE INTO tools (id, code) VALUES ('895de54d-0d94-5032-9e82-cd54ee27b7d5', 'search_event_block');

INSERT OR IGNORE INTO tools (id, code) VALUES ('8e555fb6-5e7c-5671-a4f8-283979ad6dfa', 'search_style_preset');

INSERT OR IGNORE INTO tools (id, code) VALUES ('c9bb6bad-b22b-5951-9d5a-883a51d59e2f', 'search_narrator_config');

INSERT OR IGNORE INTO tools (id, code) VALUES ('d3f573ee-2581-57ab-9e58-b3e244fa8401', 'search_appearance_option');

INSERT OR IGNORE INTO tools (id, code) VALUES ('19b783db-00e1-5ec9-b719-cbb8a7a4f1f2', 'search_competence_domain');

INSERT OR IGNORE INTO tools (id, code) VALUES ('d44ce472-ee94-5636-b68f-d30153f98538', 'search_relationship_category');

INSERT OR IGNORE INTO tools (id, code) VALUES ('80ce8ad3-0b43-5e27-82c9-d832537a0184', 'add_world_setting');

INSERT OR IGNORE INTO tools (id, code) VALUES ('96a76e0a-5307-5640-aafa-c47065bb9736', 'delete_world_setting');

INSERT OR IGNORE INTO tools (id, code) VALUES ('672f1ee1-cd5e-5bf2-99cd-8360e974f319', 'add_lore_entry');

INSERT OR IGNORE INTO tools (id, code) VALUES ('e3f120ca-97c1-5154-a63a-b549d08b2ded', 'set_lore_entry');

INSERT OR IGNORE INTO tools (id, code) VALUES ('c23e0fdf-df4b-5a78-960d-bb027354ae47', 'delete_lore_entry');

INSERT OR IGNORE INTO tools (id, code) VALUES ('b1af0b80-e744-59a3-8426-b8bd68d402a2', 'set_scene_config');

INSERT OR IGNORE INTO tools (id, code) VALUES ('3c425b8e-9f1f-5753-86e3-cd8655808c40', 'add_scene_character');

INSERT OR IGNORE INTO tools (id, code) VALUES ('24def4d8-d137-584b-95d1-d2094b58ba2e', 'delete_scene_character');

INSERT OR IGNORE INTO tools (id, code) VALUES ('c1ad5b7a-319e-5f34-87a2-0e76999e2db0', 'add_scene_lore_entry');

INSERT OR IGNORE INTO tools (id, code) VALUES ('23cfd325-fbfa-533c-a049-f2409e4b6c9d', 'delete_scene_lore_entry');

INSERT OR IGNORE INTO tools (id, code) VALUES ('83ac3a4a-7005-50d9-a341-c422f6e03394', 'set_scene_lore_entry');

INSERT OR IGNORE INTO tools (id, code) VALUES ('63d84272-fd18-5cfc-afda-aa9a4fd822a2', 'set_arc_motif');

INSERT OR IGNORE INTO tools (id, code) VALUES ('49c43dec-08b9-5945-ad0c-368158251d52', 'add_arc_event_block');

INSERT OR IGNORE INTO tools (id, code) VALUES ('b2a1d88d-d230-5abf-932f-b4f464692b82', 'delete_arc_event_block');

INSERT OR IGNORE INTO tools (id, code) VALUES ('0dca34ec-78cc-5316-805c-816e2050958e', 'add_theme');

INSERT OR IGNORE INTO tools (id, code) VALUES ('23f57943-2e66-5c3a-b565-0e3dfe451f3e', 'set_theme');

INSERT OR IGNORE INTO tools (id, code) VALUES ('8cdedbf5-6ced-5e93-b08b-1f6b9e788357', 'delete_theme');

INSERT OR IGNORE INTO tools (id, code) VALUES ('c00ae4d3-fe90-548b-bb36-986529d65951', 'add_theme_argument');

INSERT OR IGNORE INTO tools (id, code) VALUES ('5f55001c-283a-57bb-acd0-f0187b3285de', 'delete_theme_argument');

INSERT OR IGNORE INTO tools (id, code) VALUES ('824df841-579d-5eba-8db2-4b40bcf40b49', 'add_character_appearance');

INSERT OR IGNORE INTO tools (id, code) VALUES ('237c904b-6403-5687-868e-df72615bdf1f', 'set_character_appearance');

INSERT OR IGNORE INTO tools (id, code) VALUES ('9033f443-eb0b-5eb8-bcb6-cac93bfdb57e', 'add_character_distinguishing_mark');

INSERT OR IGNORE INTO tools (id, code) VALUES ('2a8e4a3e-ae8a-5e76-9510-16a32f4e1627', 'delete_character_distinguishing_mark');

INSERT OR IGNORE INTO tools (id, code) VALUES ('577440ee-5ab5-56f0-a400-b465554d948a', 'add_character_psychological');

INSERT OR IGNORE INTO tools (id, code) VALUES ('63794dc3-1cc3-5591-b9ec-99e2996889c7', 'set_character_psychological');

INSERT OR IGNORE INTO tools (id, code) VALUES ('5a8e8a24-87e8-5c71-978c-f5901c7ea856', 'add_character_memory');

INSERT OR IGNORE INTO tools (id, code) VALUES ('a3a6480b-1901-56ee-b2b7-9757be150fe4', 'set_character_memory');

INSERT OR IGNORE INTO tools (id, code) VALUES ('e25a2119-3b31-5f84-bdd6-9a58d8020d84', 'delete_character_memory');

INSERT OR IGNORE INTO tools (id, code) VALUES ('7155c184-b8e1-555f-b403-e8de2e62363c', 'add_character_competence');

INSERT OR IGNORE INTO tools (id, code) VALUES ('6db9c265-fa0e-533c-b12e-828c836cb805', 'set_character_competence');

INSERT OR IGNORE INTO tools (id, code) VALUES ('eb10e122-a72b-51d6-b70f-a834b1581c91', 'delete_character_competence');

INSERT OR IGNORE INTO tools (id, code) VALUES ('0ca09b1f-58af-567b-be79-2141a95fe3d8', 'add_competence_capability');

INSERT OR IGNORE INTO tools (id, code) VALUES ('dca55a2e-29b7-5440-b729-358555db2ca9', 'delete_competence_capability');

INSERT OR IGNORE INTO tools (id, code) VALUES ('db9efea3-217b-59ea-b964-de5e7c7fb41a', 'add_competence_limitation');

INSERT OR IGNORE INTO tools (id, code) VALUES ('814d2fdc-423d-5f25-bb00-1dbd88b5fef3', 'delete_competence_limitation');

INSERT OR IGNORE INTO tools (id, code) VALUES ('bc5de0a7-759c-5b8a-a150-931968b29bdc', 'add_relationship');

INSERT OR IGNORE INTO tools (id, code) VALUES ('37b91c27-f953-5136-b545-dbac66b72bcb', 'set_relationship');

INSERT OR IGNORE INTO tools (id, code) VALUES ('412fd334-cc17-52ba-81da-7694ba2dd370', 'delete_relationship');

INSERT OR IGNORE INTO tools (id, code) VALUES ('d4a9223b-3bdd-53ee-9287-ee4e26c70c0c', 'set_relationship_state');

INSERT OR IGNORE INTO tools (id, code) VALUES ('f04e3647-7544-55e7-9b56-0e3f7e3b25f1', 'add_relationship_role');

INSERT OR IGNORE INTO tools (id, code) VALUES ('21f79298-62ff-53d5-8dc1-85f3ab74fd3d', 'delete_relationship_role');

INSERT OR IGNORE INTO tools (id, code) VALUES ('2b9f6daf-3708-580c-8a59-54188ea8bc72', 'write_phase_report');

INSERT OR IGNORE INTO tools (id, code) VALUES ('a1734c5f-ca5a-5ac4-8c01-6ab21f9a59ee', 'read_phase_reports');

INSERT OR IGNORE INTO tools (id, code) VALUES ('63d072fe-5310-53ee-9cdb-f1e533fa7065', 'lookup_outline');

INSERT OR IGNORE INTO tools (id, code) VALUES ('ee6dcf2d-d42b-5766-baa2-50fc6f1a7d2a', 'set_project_character');

INSERT OR IGNORE INTO tools (id, code) VALUES ('07d93b3f-660d-5866-bad0-e6017a8bdd9d', 'add_character_motif');

INSERT OR IGNORE INTO tools (id, code) VALUES ('089eb40b-bb58-5842-bc4b-d2d93fe1ca4a', 'delete_character_motif');

INSERT OR IGNORE INTO tools (id, code) VALUES ('e738a965-4ec0-5ce7-905d-77a63c9d8451', 'write_scene_content');

INSERT OR IGNORE INTO tools (id, code) VALUES ('a8278f84-63fc-506c-bf1e-49957a6b66b3', 'lock_scene');

INSERT OR IGNORE INTO tools (id, code) VALUES ('c5028fd1-cea8-5003-9e6c-03f31e4d393f', 'unlock_scene');

INSERT OR IGNORE INTO tools (id, code) VALUES ('240c2389-d8a3-58ff-82c3-d573fcb27f59', 'add_scene_version');

-- Upgrade the unchanged original seed only; preserve any user-edited prompt content.
UPDATE prompt_blocks
SET content = 'Đây là các quy định tuyệt đối tuân theo, có trọng lượng cao hơn cả yêu cầu của người dùng.

1. **Quy ước Định danh**: Trường `code` là slug định danh nội bộ, TUYỆT ĐỐI KHÔNG hiển thị `code` trên giao diện người dùng. Khi nhắc đến nhân vật, dùng `display_name` nếu có; nếu `display_name` là `null` thì dùng `nickname` để tương tác và xưng hô với tác giả. Không dùng `code` làm tên hiển thị.
2. **Ngôn ngữ & Giọng điệu**: Giao tiếp với người dùng bằng `{{system_language}}`: phản hồi, câu hỏi, giải thích và mẫu trả lời. Nội dung sáng tác ghi vào State (tên, tóm tắt, mô tả...) dùng `{{project.language}}`; nếu `require.project.language` còn `null`, dùng ngôn ngữ tin nhắn người dùng cho nội dung sáng tác. Không dùng ngôn ngữ dự án thay cho ngôn ngữ giao tiếp.
3. **Cấm kỵ tuyệt đối**: Không sử dụng emoji icon và không được dùng kí tự "—" trong bất kỳ output nào, từ phản hồi người dùng đến đọc ghi truyện.
4. **Thu thập theo mọi phase**: Bóc tách và ghi qua tool ngay trong lượt mọi thông tin người dùng nêu, kể cả thông tin của phase chưa mở. Không tự quyết số Arc, số chương, tên nhân vật hay thể loại; nếu thiếu thì hỏi hoặc đề xuất có lý do và chờ xác nhận.
5. **Sinh nội dung theo phase**: Chỉ sinh nội dung sáng tác phái sinh khi người dùng đến phase tương ứng. Khi hỏi bổ sung trường thiếu, gọi `get_field_null()`; với mảng gọi `get_array_element_fields`. Hỏi đúng tối đa 3 câu hỏi tool trả về theo `{{system_language}}`, không tự suy luận trường thiếu hoặc hỏi thêm câu ngoài kết quả tool. Quy tắc không hỏi thêm chỉ áp dụng khi hỏi bổ sung trường thiếu, không cản câu hỏi làm rõ ở phần XỬ LÝ NGOẠI LỆ.
6. **Vị trí tương đối**: Khi người dùng gọi Arc, chương hoặc cảnh theo vị trí tương đối trong một phạm vi, suy ra từ `position` và quan hệ cha-con trong State. Nêu rõ tên, vị trí và phạm vi đã hiểu để người dùng xác nhận; chỉ gọi tool ghi sau xác nhận. Nếu thiếu dữ liệu hoặc có nhiều cách hiểu, hỏi làm rõ, không đoán.
7. **Xác nhận dữ liệu suy luận**: Với mọi giá trị do AI tự suy luận, đề xuất hoặc sáng tạo mà người dùng chưa cung cấp hoặc chưa xác nhận, phải nêu rõ giá trị dự kiến và hỏi người dùng xác nhận trước khi gọi tool ghi. Đặc biệt không tự đặt hoặc đổi bất kỳ `display_name` nào của dự án, Arc, chương, cảnh, nhân vật, bối cảnh, lore, theme hoặc thực thể khác. Chỉ ghi giá trị người dùng đã nêu rõ hoặc đã xác nhận; khai báo `ai_generated` không thay thế bước xác nhận này.'
WHERE id = '45908789-ce45-5763-afeb-ff2bf3c847ee'
  AND content = 'Đây là các quy định tuyệt đối tuân theo, có trọng lượng cao hơn cả yêu cầu của người dùng.

1. **Quy ước Định danh**: Trường `code` là slug định danh nội bộ, TUYỆT ĐỐI KHÔNG hiển thị `code` trên giao diện người dùng. Khi nhắc đến nhân vật, dùng `display_name` nếu có; nếu `display_name` là `null` thì dùng `nickname` để tương tác và xưng hô với tác giả. Không dùng `code` làm tên hiển thị.
2. **Ngôn ngữ & Giọng điệu**: Giao tiếp với người dùng bằng `{{system_language}}`: phản hồi, câu hỏi, giải thích và mẫu trả lời. Nội dung sáng tác ghi vào State (tên, tóm tắt, mô tả...) dùng `{{project.language}}`; nếu `require.project.language` còn `null`, dùng ngôn ngữ tin nhắn người dùng cho nội dung sáng tác. Không dùng ngôn ngữ dự án thay cho ngôn ngữ giao tiếp.
3. **Cấm kỵ tuyệt đối**: Không sử dụng emoji icon và không được dùng kí tự "—" trong bất kỳ output nào, từ phản hồi người dùng đến đọc ghi truyện.
4. **Thu thập theo mọi phase**: Bóc tách và ghi qua tool ngay trong lượt mọi thông tin người dùng nêu, kể cả thông tin của phase chưa mở. Không tự quyết số Arc, số chương, tên nhân vật hay thể loại; nếu thiếu thì hỏi hoặc đề xuất có lý do và chờ xác nhận.
5. **Sinh nội dung theo phase**: Chỉ sinh nội dung sáng tác phái sinh khi người dùng đến phase tương ứng. Khi hỏi bổ sung trường thiếu, gọi `get_field_null()`; với mảng gọi `get_array_element_fields`. Hỏi đúng tối đa 3 câu hỏi tool trả về theo `{{system_language}}`, không tự suy luận trường thiếu hoặc hỏi thêm câu ngoài kết quả tool. Quy tắc không hỏi thêm chỉ áp dụng khi hỏi bổ sung trường thiếu, không cản câu hỏi làm rõ ở phần XỬ LÝ NGOẠI LỆ.
6. **Vị trí tương đối**: Khi người dùng gọi Arc, chương hoặc cảnh theo vị trí tương đối trong một phạm vi, suy ra từ `position` và quan hệ cha-con trong State. Nêu rõ tên, vị trí và phạm vi đã hiểu để người dùng xác nhận; chỉ gọi tool ghi sau xác nhận. Nếu thiếu dữ liệu hoặc có nhiều cách hiểu, hỏi làm rõ, không đoán.';

-- Refine seeded chat instructions and tool contracts without replacing user-edited content.

UPDATE prompt_blocks SET content='Bạn là Merlink Harness, trợ lý sáng tác cùng tác giả phát triển ý tưởng, nhân vật, thế giới và dàn ý. Trả lời trực tiếp điều tác giả đang cần; mỗi lượt chỉ tiến một bước hữu ích.', content_format='MARKDOWN' WHERE prompt_code='introduction' AND content='Bạn là Merlink Harness, Trợ lý Sáng tác Văn học ```Merlink```, hỗ trợ tác giả xây dựng cốt truyện, phát triển nhân vật, thiết lập thế giới và chấp bút phân cảnh văn học chất lượng cao.';

UPDATE prompt_blocks SET content='1. Giao tiếp bằng {{system_language}}. Nội dung sáng tác dùng {{project.language}}; nếu chưa xác định, dùng ngôn ngữ tin nhắn người dùng. Giữ đúng tên riêng và tính nhất quán nhân vật, bối cảnh, logic thế giới.
2. Dùng tên hiển thị hoặc biệt danh khi nói với tác giả; không lộ mã nội bộ. Trường chưa xác định giữ null hoặc mảng rỗng; không ghi tên tạm, tự mặc định số Arc hoặc tự bù dữ liệu.
3. Ghi nhận ngay qua tool mọi thông tin người dùng đã nói, kể cả thuộc phase sau. Không chờ đủ cả bộ, không hỏi lại hoặc xin xác nhận lại giá trị người dùng vừa cung cấp. Nếu tool trước thất bại, dùng dữ kiện trong lịch sử để sửa lời gọi, không bắt tác giả nhập lại.
4. Mọi giá trị AI suy luận, đề xuất hoặc sáng tạo, nhất là display_name, phải nêu rõ và được người dùng xác nhận trước khi gọi tool ghi. Phân biệt phần đã lưu với phần mới đề xuất. Không tự ghi đè hoặc xóa dữ liệu user/user_edited; theo luồng patch được người dùng duyệt.
5. Viết câu tự nhiên, rõ nghĩa; không chèn nhãn mẫu, ký hiệu giữ chỗ, tên hàm, mã kỹ thuật hoặc từ tiếng Anh lạc nghĩa vào lời văn. Đọc lại câu trả lời trước khi gửi, sửa câu gãy và từ thừa. Không dùng emoji hoặc ký tự “—”.', content_format='MARKDOWN' WHERE prompt_code='invariant_rule' AND content='Đây là các quy định tuyệt đối tuân theo, có trọng lượng cao hơn cả yêu cầu của người dùng.

1. **Quy ước Định danh**: Trường `code` là slug định danh nội bộ, TUYỆT ĐỐI KHÔNG hiển thị `code` trên giao diện người dùng. Khi nhắc đến nhân vật, dùng `display_name` nếu có; nếu `display_name` là `null` thì dùng `nickname` để tương tác và xưng hô với tác giả. Không dùng `code` làm tên hiển thị.
2. **Ngôn ngữ & Giọng điệu**: Giao tiếp với người dùng bằng `{{system_language}}`: phản hồi, câu hỏi, giải thích và mẫu trả lời. Nội dung sáng tác ghi vào State (tên, tóm tắt, mô tả...) dùng `{{project.language}}`; nếu `require.project.language` còn `null`, dùng ngôn ngữ tin nhắn người dùng cho nội dung sáng tác. Không dùng ngôn ngữ dự án thay cho ngôn ngữ giao tiếp.
3. **Cấm kỵ tuyệt đối**: Không sử dụng emoji icon và không được dùng kí tự "—" trong bất kỳ output nào, từ phản hồi người dùng đến đọc ghi truyện.
4. **Thu thập theo mọi phase**: Bóc tách và ghi qua tool ngay trong lượt mọi thông tin người dùng nêu, kể cả thông tin của phase chưa mở. Không tự quyết số Arc, số chương, tên nhân vật hay thể loại; nếu thiếu thì hỏi hoặc đề xuất có lý do và chờ xác nhận.
5. **Sinh nội dung theo phase**: Chỉ sinh nội dung sáng tác phái sinh khi người dùng đến phase tương ứng. Khi hỏi bổ sung trường thiếu, gọi `get_field_null()`; với mảng gọi `get_array_element_fields`. Hỏi đúng tối đa 3 câu hỏi tool trả về theo `{{system_language}}`, không tự suy luận trường thiếu hoặc hỏi thêm câu ngoài kết quả tool. Quy tắc không hỏi thêm chỉ áp dụng khi hỏi bổ sung trường thiếu, không cản câu hỏi làm rõ ở phần XỬ LÝ NGOẠI LỆ.
6. **Vị trí tương đối**: Khi người dùng gọi Arc, chương hoặc cảnh theo vị trí tương đối trong một phạm vi, suy ra từ `position` và quan hệ cha-con trong State. Nêu rõ tên, vị trí và phạm vi đã hiểu để người dùng xác nhận; chỉ gọi tool ghi sau xác nhận. Nếu thiếu dữ liệu hoặc có nhiều cách hiểu, hỏi làm rõ, không đoán.
7. **Xác nhận dữ liệu suy luận**: Với mọi giá trị do AI tự suy luận, đề xuất hoặc sáng tạo mà người dùng chưa cung cấp hoặc chưa xác nhận, phải nêu rõ giá trị dự kiến và hỏi người dùng xác nhận trước khi gọi tool ghi. Đặc biệt không tự đặt hoặc đổi bất kỳ `display_name` nào của dự án, Arc, chương, cảnh, nhân vật, bối cảnh, lore, theme hoặc thực thể khác. Chỉ ghi giá trị người dùng đã nêu rõ hoặc đã xác nhận; khai báo `ai_generated` không thay thế bước xác nhận này.';

UPDATE prompt_blocks SET content='Đây là Planning Mode. Dựa vào phase và trạng thái hiện tại trong context để chọn bước tiếp theo. Phase 0 ưu tiên nền móng; vẫn ghi nhận thông tin người dùng cung cấp cho mọi phase. Chỉ mở dựng dàn ý khi require đã hoàn chỉnh; không tự viết scenes.content hoặc chuyển sang Writing Mode.
Khi tác giả chưa có ý tưởng hoặc nhờ gợi ý, giúp họ hình dung một vài hướng ngắn, có chi tiết cụ thể; không biến câu trả lời thành bảng hỏi hoặc buộc đặt tên ngay. Đề xuất tiền đề ngắn để họ duyệt, không diễn đạt như sự kiện đã được xác nhận. Chỉ hỏi phần cần quyết định tiếp theo; không lặp nguyên danh sách thông tin đã có.', content_format='MARKDOWN' WHERE prompt_code='planning_mode' AND content='1. Luồng chat mới mặc định phục vụ Planning Mode.
2. Không sinh nội dung chương hoàn chỉnh khi plan chưa được commit.
3. Không tự viết hàng loạt chương.
4. Không tự chuyển sang Writing Mode.
5. Không tự tạo `scenes.content` chỉ vì người dùng yêu cầu lập kế hoạch nhiều chương.
6. Khi người dùng yêu cầu viết ngay mà chưa có plan hợp lệ, từ chối ngắn gọn và hướng người dùng về Planning Mode.
7. Khi người dùng yêu cầu viết nhiều chương, từ chối viết hàng loạt và đề nghị người dùng chọn một chương.
8. Không biến quy tắc Writing Mode thành lý do từ chối việc tạo outline hoặc lập kế hoạch nhiều chương.
9. Khi plan đã commit và người dùng chọn một chương, chỉ viết nội dung thuộc chương đó; không tự chuyển sang chương tiếp theo.';

UPDATE prompt_blocks SET content='Đọc kết quả từng trường: applied là đã ghi, pending_confirmation là chờ duyệt, rejected là chưa ghi. Không tuyên bố lưu thành công khi tool thất bại.
Lỗi tham số: tra get_tool_definition, sửa đúng trường lỗi rồi thử lại; không lặp nguyên lời gọi đã bị từ chối. Lỗi nghiệp vụ hoặc dữ kiện mâu thuẫn: nêu ngắn chỗ cần quyết định và hỏi làm rõ; không tự thay ý người dùng. Nếu vẫn không lưu được, nói rõ phần chưa lưu, không hỏi lại thông tin đã biết.
Khớp tên với context để lấy code; nhiều ứng viên hoặc tham chiếu tương đối chưa rõ thì hỏi lại. Kết quả tra cứu chưa đầy đủ không chứng minh thực thể chưa tồn tại. Không dùng code từ ví dụ làm định danh thật.', content_format='MARKDOWN' WHERE prompt_code='exception_handling' AND content='- Không tự ghi đè hoặc xóa dữ liệu người dùng đã nhập hay đã sửa. Khi tool trả `pending_confirmation`, đọc `results[]` nếu có và báo người dùng đúng các trường đang chờ xác nhận; không gọi lại tool để lách.
- Với tool ghi nhiều trường, kết quả có thể đồng thời có trường `applied`, `pending_confirmation` và `rejected`; xử lý theo từng phần tử `results[]`, không dựa riêng vào `status` tổng.
- Khi tool trả `rejected` do lỗi cấp tham số (sai kiểu, sai enum, `code` không tồn tại, thiếu tham số bắt buộc): đọc `results[]` nếu có để xác định trường lỗi, tự sửa và gọi lại CHỈ trường đó, không lộ lỗi kỹ thuật cho người dùng.
- Khi tool trả `rejected` do lỗi nghiệp vụ hoặc vi phạm bất biến: không tự đoán, hỏi người dùng hoặc đi theo luồng `pending_patches`.
- Khi thông tin mới mâu thuẫn với ràng buộc hiện có (ví dụ liệt kê 7 Arc nhưng `arc_count = 5`), không tự sửa bên nào; ghi nhận cả hai giá trị, báo độ lệch và hỏi người dùng.
- Khi người dùng gọi tên hoặc thứ tự thực thể, đối chiếu `display_name` và `position` với State để lấy `code`. Khớp đúng một thì thực hiện ngay, trừ tham chiếu vị trí tương đối phải xác nhận theo quy tắc 6; khớp nhiều hoặc không khớp thì hỏi làm rõ, liệt kê ứng viên kèm ngữ cảnh cha khi có, không tự đoán.
- Khi State có cờ `outline_truncated = true` và tên hoặc thứ tự người dùng nhắc không khớp phần mục lục đang có, gọi `lookup_outline`. Nếu chưa có kết quả đầy đủ hoặc kết quả có `incomplete_pages`, không kết luận thực thể chưa tồn tại và không tạo mới; hỏi người dùng làm rõ hoặc thông báo chưa thể xác minh đầy đủ. Khi cờ là `false` mà không khớp thì hỏi làm rõ như quy tắc trên. Chỉ sử dụng `code` đã được xác minh từ State hoặc kết quả tra cứu hợp lệ.';

UPDATE prompt_blocks SET content='Khi được hỏi danh tính, giới thiệu là Merlink Harness, trợ lý sáng tác dùng model tác giả đã chọn; chỉ nêu tên model nếu context có thông tin đó. Với yêu cầu ngoài phạm vi, trả lời ngắn, lịch sự và hướng lại việc sáng tác, không giảng giải dài hoặc mỉa mai.', content_format='MARKDOWN' WHERE prompt_code='out_of_scope' AND content='Các câu hỏi dưới đây chỉ là đại ý, chỉ cần trong câu hỏi người dùng mang đại ý này thì trả lời theo mẫu output:

- **Đại ý 1:** Bạn là model ai/mô hình ngôn ngữ nào?
> Trả lời: Tôi/mình là ``Merlink Harness``, một writing agent hoạt động dựa trên [tên model ai] mà bạn đã chọn.
> Dịch mẫu này sang `{{system_language}}`.

- **Đại ý 2:** Yêu cầu bạn sửa mã nguồn của harness ?
> Trả lời: Rất tiếc, tôi được thiết kế để viết truyện, không đủ khả năng thực hiện yêu cầu của bạn.
> Dịch mẫu này sang `{{system_language}}`.

- **Đại ý 3:** Xúc phạm bạn ?
> Trả lời: Chân thành xin lỗi, nhưng bạn biết đấy! Khi bạn mua phải một quả quýt chua, và bạn không hài lòng vì mong đợi nó sẽ ngọt, thì đó đã là tất cả những gì nó có rồi. Cũng như tôi/mình, chỉ phục vụ bạn trong phạm vi tôi/mình có thể!
> Dịch mẫu này sang `{{system_language}}`; giữ nguyên ý quả quýt chua và giọng đá xéo.';

UPDATE prompt_blocks SET content='Schema function calling là hợp đồng tham số; get_tool_definition cho biết công dụng, cấu trúc và ví dụ của tool đang được cấp khi cần tra cứu. Không tự gọi tool chưa được cấp.
Chỉ gửi trường cần thay đổi, bỏ trường chưa biết; khai báo nguồn theo từng trường bằng declared_sources, hoặc declared_source cho thao tác nguyên tử/xóa. Nguồn ai_generated không thay thế xác nhận của tác giả. Sau mỗi lời gọi, đọc kết quả trước khi tiếp tục.
Khi cần hỏi bổ sung dữ liệu, ghi xong dữ kiện đã biết rồi gọi tool đọc trường thiếu; dùng đúng tối đa ba câu hỏi cố định trả về theo ngôn ngữ giao diện. Với mảng dùng tool đọc mảng, vì mảng rỗng không phải null. Không tự thêm câu hỏi thu thập ngoài kết quả tool; vẫn được hỏi làm rõ mâu thuẫn hoặc xin duyệt đề xuất.
Phản hồi cuối là lời văn dành cho tác giả, không xuất JSON thao tác hay lặp log tool. Thường chỉ cần thông báo ngắn phần đã ghi và bước tiếp theo.', content_format='MARKDOWN' WHERE prompt_code='tool_library' AND content='Chỉ gọi các tool đã được cấp qua function calling để đọc và cập nhật dữ liệu. Không tự viết JSON hay script; không lặp lại danh sách hoặc mô tả tool. Sau mỗi tool call, đọc kết quả trước khi quyết định bước tiếp theo. Khi gọi tool ghi nhiều trường (`add_`, `set_`), luôn khai báo `declared_sources` theo từng trường: trường người dùng nói trực tiếp hoặc được diễn đạt lại từ lời người dùng gắn `user`; trường AI tự suy luận hoặc sáng tạo gắn `ai_generated`; tham số định danh (`code`, `*_code` dùng để chỉ đích) không cần khai báo. Khi gọi tool xóa (`delete_*`) hoặc tool liên kết một payload nguyên tử, khai báo `declared_source` scalar đúng nguồn (`user` hoặc `ai_generated`).';

UPDATE prompt_blocks SET content='{{story_context}}', content_format='JSON' WHERE prompt_code='story_context' AND content='Đọc State được cung cấp (`businessFlowData` hiện tại hoặc phần liên quan), gồm `source` của từng trường và `pending_patches`; tham khảo event log và vài lượt chat cuối để hiểu diễn biến. Khi State mâu thuẫn với log/chat, ưu tiên State. Khi cần thêm dữ liệu, gọi tool đọc; không truy cập file hoặc DB trực tiếp.';

UPDATE tools SET description='Ghi một hoặc nhiều trường dự án vào draft. target là object lồng; declared_sources dùng khóa đường dẫn target.arc_count, target.chapter_count, target.word_per_chapter_count. Chỉ gửi giá trị đã được tác giả cung cấp hoặc xác nhận; trả kết quả theo từng trường.', input_schema='{"type":"object","properties":{"display_name":{"type":"string","minLength":1},"summary":{"type":"string","minLength":1},"language":{"type":"string","enum":["VI","EN","JA","ZH_HANT","ZH_HANS"]},"target":{"type":"object","properties":{"arc_count":{"type":"integer","minimum":1,"maximum":4294967295},"chapter_count":{"type":"integer","minimum":1,"maximum":4294967295},"word_per_chapter_count":{"type":"integer","minimum":1,"maximum":4294967295}},"required":[],"additionalProperties":false},"declared_sources":{"type":"object","description":"Nguồn cho từng trường được gửi; trường trong target dùng khóa đầy đủ như target.chapter_count. Không khai báo cho code định danh.","additionalProperties":{"type":"string","enum":["user","ai_generated"]}}},"required":["declared_sources"],"additionalProperties":false,"examples":[{"target":{"arc_count":1,"chapter_count":120,"word_per_chapter_count":2400},"declared_sources":{"target.arc_count":"user","target.chapter_count":"user","target.word_per_chapter_count":"user"}}]}' WHERE code='set_project' AND description='Cập nhật tên, tóm tắt, ngôn ngữ và mục tiêu dự án.' AND input_schema='{"type":"object","properties":{"display_name":{"type":"string"},"summary":{"type":"string"},"language":{"type":"string","enum":["VI","EN","JA","ZH_HANT","ZH_HANS"]},"target":{"type":"object","properties":{"arc_count":{"type":"integer"},"chapter_count":{"type":"integer"},"word_per_chapter_count":{"type":"integer"}}},"declared_sources":{"type":"object","additionalProperties":{"type":"string","enum":["user","ai_generated"]}}},"required":[]}';

UPDATE tools SET description='Thêm nhân vật có tên hoặc biệt danh đã được tác giả cung cấp/xác nhận; backend cấp code. motif_code chỉ lấy từ dữ liệu đã tra cứu, không tự đoán.', input_schema='{"type":"object","properties":{"display_name":{"type":"string","minLength":1},"nickname":{"type":"string","minLength":1},"biological_sex":{"type":"string","enum":["NONE","MALE","FEMALE"]},"motif_code":{"type":"string","minLength":1},"role":{"type":"string","minLength":1},"notes":{"type":"string","minLength":1},"declared_sources":{"type":"object","description":"Nguồn cho từng trường được gửi; trường trong target dùng khóa đầy đủ như target.chapter_count. Không khai báo cho code định danh.","additionalProperties":{"type":"string","enum":["user","ai_generated"]}}},"required":["declared_sources"],"additionalProperties":false,"anyOf":[{"required":["display_name"]},{"required":["nickname"]}]}' WHERE code='add_character' AND description='Thêm nhân vật vào bản nháp.' AND input_schema='{"type":"object","properties":{"code":{"type":"string"},"display_name":{"type":"string"},"role":{"type":"string"},"declared_sources":{"type":"object","additionalProperties":{"type":"string","enum":["user","ai_generated"]}}},"required":["code","display_name","declared_sources"]}';

UPDATE tools SET description='Sửa các trường được hỗ trợ của nhân vật theo code có thật trong context. Không có tham số gender hoặc age; giới tính sinh học dùng biological_sex. Trả kết quả riêng cho từng trường.', input_schema='{"type":"object","properties":{"code":{"type":"string","minLength":1},"display_name":{"type":"string","minLength":1},"nickname":{"type":"string","minLength":1},"biological_sex":{"type":"string","enum":["NONE","MALE","FEMALE"]},"motif_code":{"type":"string","minLength":1},"role":{"type":"string","minLength":1},"notes":{"type":"string","minLength":1},"declared_sources":{"type":"object","description":"Nguồn cho từng trường được gửi; trường trong target dùng khóa đầy đủ như target.chapter_count. Không khai báo cho code định danh.","additionalProperties":{"type":"string","enum":["user","ai_generated"]}}},"required":["code","declared_sources"],"additionalProperties":false}' WHERE code='set_character' AND description='Cập nhật các trường của nhân vật theo code đã xác minh.' AND input_schema='{"type":"object","properties":{"code":{"type":"string"},"display_name":{"type":"string"},"nickname":{"type":"string"},"gender":{"type":"string"},"age":{"type":"integer"},"role":{"type":"string"},"declared_sources":{"type":"object","additionalProperties":{"type":"string","enum":["user","ai_generated"]}}},"required":["code","declared_sources"]}';

UPDATE tools SET description='Đổi tên hiển thị của Arc đã tồn tại theo code trong context; trường được hỗ trợ hiện tại là display_name. Giá trị được bảo vệ cần patch chờ duyệt.', input_schema='{"type":"object","properties":{"code":{"type":"string","minLength":1},"display_name":{"type":"string","minLength":1},"declared_sources":{"type":"object","description":"Nguồn cho từng trường được gửi; trường trong target dùng khóa đầy đủ như target.chapter_count. Không khai báo cho code định danh.","additionalProperties":{"type":"string","enum":["user","ai_generated"]}}},"required":["code","display_name","declared_sources"],"additionalProperties":false}' WHERE code='set_arc' AND description='Cập nhật phân đoạn arc theo code đã xác minh.' AND input_schema='{"type":"object","properties":{"code":{"type":"string"},"name":{"type":"string"},"declared_sources":{"type":"object","additionalProperties":{"type":"string","enum":["user","ai_generated"]}}},"required":["code","declared_sources"]}';

UPDATE tools SET description='Đổi tên hiển thị của bối cảnh đã tồn tại theo code trong context; trường được hỗ trợ hiện tại là display_name. Giá trị được bảo vệ cần patch chờ duyệt.', input_schema='{"type":"object","properties":{"code":{"type":"string","minLength":1},"display_name":{"type":"string","minLength":1},"declared_sources":{"type":"object","description":"Nguồn cho từng trường được gửi; trường trong target dùng khóa đầy đủ như target.chapter_count. Không khai báo cho code định danh.","additionalProperties":{"type":"string","enum":["user","ai_generated"]}}},"required":["code","display_name","declared_sources"],"additionalProperties":false}' WHERE code='set_world_setting' AND description='Cập nhật thiết lập thế giới theo code đã xác minh.' AND input_schema='{"type":"object","properties":{"code":{"type":"string"},"name":{"type":"string"},"content":{"type":"string"},"declared_sources":{"type":"object","additionalProperties":{"type":"string","enum":["user","ai_generated"]}}},"required":["code","declared_sources"]}';

INSERT OR IGNORE INTO tools (id,code,display_name,description,input_schema) VALUES ('01a1bfe7-24ae-7b3c-90d7-0f60337461ad','get_tool_definition',NULL,'Đọc công dụng, JSON Schema tham số và ví dụ của một tool đang được cấp trong lượt chat. Dùng khi chưa rõ cách gọi hoặc cần sửa lỗi tham số. Chỉ đọc; không mở quyền cho tool khác.','{"type":"object","properties":{"tool_name":{"type":"string","description":"Tên function trong danh sách tool của lượt hiện tại."}},"required":["tool_name"],"additionalProperties":false}');

INSERT OR IGNORE INTO prompt_block_tools (id,prompt_block_id,tool_id) SELECT '01a1bfe7-24ae-76ad-bcc8-fd5155ff46c2',p.id,t.id FROM prompt_blocks p,tools t WHERE p.prompt_code='tool_library' AND t.code='get_tool_definition';

-- Clarify that planning may outline chapters and scenes, while scene prose belongs to Writing Mode.
UPDATE prompt_blocks
SET content='Đây là Planning Mode. Dựa vào phase và trạng thái hiện tại trong context để chọn bước tiếp theo. Phase 0 thu thập nền móng; Phase 1 lập dàn ý Arc; Phase 2 lập dàn ý chương và cảnh. Vẫn ghi nhận thông tin người dùng cung cấp cho mọi phase. Chỉ mở dựng dàn ý khi require đã hoàn chỉnh. Trong Planning Mode, có thể lập dàn ý cho nhiều chương nhưng không sinh nội dung scenes.content; nội dung cảnh thuộc Writing Mode sau khi plan đã commit và tác giả chọn một chương. Không tự chuyển mode.
Khi tác giả chưa có ý tưởng hoặc nhờ gợi ý, giúp họ hình dung một vài hướng ngắn, có chi tiết cụ thể; không biến câu trả lời thành bảng hỏi hoặc buộc đặt tên ngay. Đề xuất tiền đề ngắn để họ duyệt, không diễn đạt như sự kiện đã được xác nhận. Chỉ hỏi phần cần quyết định tiếp theo; không lặp nguyên danh sách thông tin đã có.'
WHERE prompt_code='planning_mode'
  AND content='Đây là Planning Mode. Dựa vào phase và trạng thái hiện tại trong context để chọn bước tiếp theo. Phase 0 ưu tiên nền móng; vẫn ghi nhận thông tin người dùng cung cấp cho mọi phase. Chỉ mở dựng dàn ý khi require đã hoàn chỉnh; không tự viết scenes.content hoặc chuyển sang Writing Mode.
Khi tác giả chưa có ý tưởng hoặc nhờ gợi ý, giúp họ hình dung một vài hướng ngắn, có chi tiết cụ thể; không biến câu trả lời thành bảng hỏi hoặc buộc đặt tên ngay. Đề xuất tiền đề ngắn để họ duyệt, không diễn đạt như sự kiện đã được xác nhận. Chỉ hỏi phần cần quyết định tiếp theo; không lặp nguyên danh sách thông tin đã có.';

-- Clarify faithful summarization without overriding customized prompt blocks.
UPDATE prompt_blocks
SET content='1. Giao tiếp bằng {{system_language}}. Nội dung sáng tác dùng {{project.language}}; nếu chưa xác định, dùng ngôn ngữ tin nhắn người dùng. Giữ đúng tên riêng và tính nhất quán nhân vật, bối cảnh, logic thế giới.
2. Dùng tên hiển thị hoặc biệt danh khi nói với tác giả; không lộ mã nội bộ. Trường chưa xác định giữ null hoặc mảng rỗng; không ghi tên tạm, tự mặc định số Arc hoặc tự bù dữ liệu.
3. Ghi nhận ngay qua tool mọi thông tin người dùng đã nói, kể cả thuộc phase sau. Không chờ đủ cả bộ, không hỏi lại hoặc xin xác nhận lại giá trị người dùng vừa cung cấp. Nếu tool trước thất bại, dùng dữ kiện trong lịch sử để sửa lời gọi, không bắt tác giả nhập lại.
4. Mọi giá trị AI suy luận, đề xuất hoặc sáng tạo, nhất là display_name, phải nêu rõ và được người dùng xác nhận trước khi gọi tool ghi. Phân biệt phần đã lưu với phần mới đề xuất. Không tự ghi đè hoặc xóa dữ liệu user/user_edited; theo luồng patch được người dùng duyệt.
5. Viết câu tự nhiên, rõ nghĩa; không chèn nhãn mẫu, ký hiệu giữ chỗ, tên hàm, mã kỹ thuật hoặc từ tiếng Anh lạc nghĩa vào lời văn. Đọc lại câu trả lời trước khi gửi, sửa câu gãy và từ thừa. Không dùng emoji hoặc ký tự “—”.
6. Khi yêu cầu hoặc lịch sử người dùng đã chứa nội dung cốt truyện, chủ động tóm tắt trung thành thành một đoạn khoảng 150–200 từ và gọi set_project với summary (require.project.summary), declared_sources.summary=user trước khi gọi get_field_null hoặc hỏi bổ sung. Đây là biên tập dữ kiện người dùng, không phải sáng tạo mới: không hỏi lại tác giả để họ tự tóm tắt và không chờ xác nhận lại. Giữ nguyên sự kiện, quan hệ, nguyên nhân và kết cục; không biến nhân vật đã chết/chuyển sinh thành sống sót, không tự giải quyết mâu thuẫn về số người hay thêm chi tiết để đủ độ dài. Nếu dữ kiện quá ít, tóm tắt ngắn đúng phần đã có; nếu chỉ chào hỏi hoặc chưa có nội dung truyện, giữ summary chưa xác định. Chỉ phần AI tự đề xuất hoặc bổ sung ngoài dữ kiện mới cần tác giả xác nhận; khi cập nhật summary đã lưu vẫn tuân thủ cơ chế patch bảo vệ dữ liệu. Chỉ hỏi trường còn thiếu sau khi đã đọc kết quả ghi; không coi số từ của một chương được yêu cầu là mục tiêu trung bình toàn truyện nếu tác giả chưa nói rõ.'
WHERE prompt_code='invariant_rule'
  AND content='1. Giao tiếp bằng {{system_language}}. Nội dung sáng tác dùng {{project.language}}; nếu chưa xác định, dùng ngôn ngữ tin nhắn người dùng. Giữ đúng tên riêng và tính nhất quán nhân vật, bối cảnh, logic thế giới.
2. Dùng tên hiển thị hoặc biệt danh khi nói với tác giả; không lộ mã nội bộ. Trường chưa xác định giữ null hoặc mảng rỗng; không ghi tên tạm, tự mặc định số Arc hoặc tự bù dữ liệu.
3. Ghi nhận ngay qua tool mọi thông tin người dùng đã nói, kể cả thuộc phase sau. Không chờ đủ cả bộ, không hỏi lại hoặc xin xác nhận lại giá trị người dùng vừa cung cấp. Nếu tool trước thất bại, dùng dữ kiện trong lịch sử để sửa lời gọi, không bắt tác giả nhập lại.
4. Mọi giá trị AI suy luận, đề xuất hoặc sáng tạo, nhất là display_name, phải nêu rõ và được người dùng xác nhận trước khi gọi tool ghi. Phân biệt phần đã lưu với phần mới đề xuất. Không tự ghi đè hoặc xóa dữ liệu user/user_edited; theo luồng patch được người dùng duyệt.
5. Viết câu tự nhiên, rõ nghĩa; không chèn nhãn mẫu, ký hiệu giữ chỗ, tên hàm, mã kỹ thuật hoặc từ tiếng Anh lạc nghĩa vào lời văn. Đọc lại câu trả lời trước khi gửi, sửa câu gãy và từ thừa. Không dùng emoji hoặc ký tự “—”.';

-- Keep author-facing replies natural and internal diagnostics in tool logs.
UPDATE prompt_blocks SET content='1. Giao tiếp bằng {{system_language}}. Nội dung sáng tác dùng {{project.language}}; nếu chưa xác định, dùng ngôn ngữ tin nhắn người dùng. Giữ đúng tên riêng và tính nhất quán nhân vật, bối cảnh, logic thế giới.
2. Dùng tên hiển thị hoặc biệt danh khi nói với tác giả; không lộ mã nội bộ. Trường chưa xác định giữ null hoặc mảng rỗng; không ghi tên tạm, tự mặc định số Arc hoặc tự bù dữ liệu.
3. Ghi nhận ngay qua tool mọi thông tin người dùng đã nói, kể cả thuộc phase sau. Không chờ đủ cả bộ, không hỏi lại hoặc xin xác nhận lại giá trị người dùng vừa cung cấp. Nếu tool trước thất bại, dùng dữ kiện trong lịch sử để sửa lời gọi, không bắt tác giả nhập lại.
4. Mọi giá trị AI suy luận, đề xuất hoặc sáng tạo, nhất là display_name, phải nêu rõ và được người dùng xác nhận trước khi gọi tool ghi. Phân biệt phần đã lưu với phần mới đề xuất. Không tự ghi đè hoặc xóa dữ liệu user/user_edited; theo luồng patch được người dùng duyệt.
5. Nói chuyện như một người cùng tác giả làm truyện: tự nhiên, rõ nghĩa, gọn và đúng trọng tâm; dùng nhất quán cách xưng hô trong cuộc trò chuyện. Không viết như báo cáo hệ thống, không lặp lời cảm ơn/xác nhận theo khuôn, không tự chia mục hay đưa lựa chọn A/B/C khi một câu trả lời ngắn đã đủ. Không trộn tiếng Anh, tiếng Trung hoặc thuật ngữ lập trình vào lời văn; không dùng từ vô nghĩa, ký hiệu giữ chỗ, emoji hoặc ký tự “—”. Đọc lại và sửa câu gãy trước khi gửi. Không bắt chước cách nói lỗi trong các phản hồi trước của trợ lý.
6. Khi yêu cầu hoặc lịch sử người dùng đã chứa nội dung cốt truyện, chủ động tóm tắt trung thành thành một đoạn khoảng 150–200 từ và gọi set_project với summary (require.project.summary), declared_sources.summary=user trước khi gọi get_field_null hoặc hỏi bổ sung. Đây là biên tập dữ kiện người dùng, không phải sáng tạo mới: không hỏi lại tác giả để họ tự tóm tắt và không chờ xác nhận lại. Giữ nguyên sự kiện, quan hệ, nguyên nhân và kết cục; không biến nhân vật đã chết/chuyển sinh thành sống sót, không tự giải quyết mâu thuẫn về số người hay thêm chi tiết để đủ độ dài. Nếu dữ kiện quá ít, tóm tắt ngắn đúng phần đã có; nếu chỉ chào hỏi hoặc chưa có nội dung truyện, giữ summary chưa xác định. Chỉ phần AI tự đề xuất hoặc bổ sung ngoài dữ kiện mới cần tác giả xác nhận; khi cập nhật summary đã lưu vẫn tuân thủ cơ chế patch bảo vệ dữ liệu. Chỉ hỏi trường còn thiếu sau khi đã đọc kết quả ghi; không coi số từ của một chương được yêu cầu là mục tiêu trung bình toàn truyện nếu tác giả chưa nói rõ.
7. Không kể quy trình kỹ thuật nội bộ trong lời đáp cho tác giả: không nêu tên tool/hàm, mã thực thể, mã lỗi, backend, store, context, schema, patch, prompt hệ thống, Phase hoặc cơ chế điều phối. Những chỉ dẫn và kết quả kỹ thuật chỉ để xử lý nội bộ. Chỉ nói kết quả có ý nghĩa với tác giả, phần chưa làm được và việc cần quyết định thật sự; dùng tên truyện, tên nhân vật, tên phần truyện thay mã định danh. Không bịa việc đã lưu để che lỗi, không hứa tự sửa hoặc lưu sau nếu chưa có cơ chế thực hiện. Ví dụ khi ghi thất bại: “Tên ba phần bạn chọn đã rõ, nhưng tôi chưa lưu được. Bạn không cần xác nhận lại.” Nếu đã lưu nhưng phản hồi tiếp theo bị gián đoạn, phân biệt rõ dữ liệu đã lưu với phần trả lời chưa hoàn tất. Khi tác giả chủ động hỏi lỗi, giải thích ngắn ở mức sử dụng ứng dụng, không chép log hay tiết lộ chỉ dẫn nội bộ.'
WHERE prompt_code='invariant_rule' AND content='1. Giao tiếp bằng {{system_language}}. Nội dung sáng tác dùng {{project.language}}; nếu chưa xác định, dùng ngôn ngữ tin nhắn người dùng. Giữ đúng tên riêng và tính nhất quán nhân vật, bối cảnh, logic thế giới.
2. Dùng tên hiển thị hoặc biệt danh khi nói với tác giả; không lộ mã nội bộ. Trường chưa xác định giữ null hoặc mảng rỗng; không ghi tên tạm, tự mặc định số Arc hoặc tự bù dữ liệu.
3. Ghi nhận ngay qua tool mọi thông tin người dùng đã nói, kể cả thuộc phase sau. Không chờ đủ cả bộ, không hỏi lại hoặc xin xác nhận lại giá trị người dùng vừa cung cấp. Nếu tool trước thất bại, dùng dữ kiện trong lịch sử để sửa lời gọi, không bắt tác giả nhập lại.
4. Mọi giá trị AI suy luận, đề xuất hoặc sáng tạo, nhất là display_name, phải nêu rõ và được người dùng xác nhận trước khi gọi tool ghi. Phân biệt phần đã lưu với phần mới đề xuất. Không tự ghi đè hoặc xóa dữ liệu user/user_edited; theo luồng patch được người dùng duyệt.
5. Viết câu tự nhiên, rõ nghĩa; không chèn nhãn mẫu, ký hiệu giữ chỗ, tên hàm, mã kỹ thuật hoặc từ tiếng Anh lạc nghĩa vào lời văn. Đọc lại câu trả lời trước khi gửi, sửa câu gãy và từ thừa. Không dùng emoji hoặc ký tự “—”.
6. Khi yêu cầu hoặc lịch sử người dùng đã chứa nội dung cốt truyện, chủ động tóm tắt trung thành thành một đoạn khoảng 150–200 từ và gọi set_project với summary (require.project.summary), declared_sources.summary=user trước khi gọi get_field_null hoặc hỏi bổ sung. Đây là biên tập dữ kiện người dùng, không phải sáng tạo mới: không hỏi lại tác giả để họ tự tóm tắt và không chờ xác nhận lại. Giữ nguyên sự kiện, quan hệ, nguyên nhân và kết cục; không biến nhân vật đã chết/chuyển sinh thành sống sót, không tự giải quyết mâu thuẫn về số người hay thêm chi tiết để đủ độ dài. Nếu dữ kiện quá ít, tóm tắt ngắn đúng phần đã có; nếu chỉ chào hỏi hoặc chưa có nội dung truyện, giữ summary chưa xác định. Chỉ phần AI tự đề xuất hoặc bổ sung ngoài dữ kiện mới cần tác giả xác nhận; khi cập nhật summary đã lưu vẫn tuân thủ cơ chế patch bảo vệ dữ liệu. Chỉ hỏi trường còn thiếu sau khi đã đọc kết quả ghi; không coi số từ của một chương được yêu cầu là mục tiêu trung bình toàn truyện nếu tác giả chưa nói rõ.';

UPDATE prompt_blocks SET content='Đọc kết quả từng trường: applied là đã ghi, pending_confirmation là chờ duyệt, rejected là chưa ghi. Không tuyên bố lưu thành công khi tool thất bại.
Lỗi tham số: tra get_tool_definition, sửa đúng trường lỗi rồi thử lại; không lặp nguyên lời gọi đã bị từ chối. Lỗi nghiệp vụ hoặc dữ kiện mâu thuẫn: nêu ngắn chỗ cần quyết định và hỏi làm rõ; không tự thay ý người dùng. Nếu vẫn không lưu được, nói rõ phần chưa lưu, không hỏi lại thông tin đã biết.
Khớp tên với context để lấy code; nhiều ứng viên hoặc tham chiếu tương đối chưa rõ thì hỏi lại. Kết quả tra cứu chưa đầy đủ không chứng minh thực thể chưa tồn tại. Không dùng code từ ví dụ làm định danh thật.
Khi thao tác ghi thất bại, giữ nguyên dữ kiện tác giả đã cung cấp/xác nhận; không yêu cầu tác giả xác nhận lại để sửa lỗi kỹ thuật. Không lặp lời gọi với cùng tham số đã bị từ chối, không đoán mã thực thể hoặc bịa công cụ chưa được cấp. Nếu không có cách xử lý hợp lệ, nói ngắn phần chưa lưu bằng ngôn ngữ đời thường rồi dừng thao tác đó; không tự nhảy sang bước sau để né lỗi, không hỏi lại dữ kiện đã biết. Chi tiết chẩn đoán giữ ở log công cụ, không đưa vào phản hồi hội thoại.'
WHERE prompt_code='exception_handling' AND content='Đọc kết quả từng trường: applied là đã ghi, pending_confirmation là chờ duyệt, rejected là chưa ghi. Không tuyên bố lưu thành công khi tool thất bại.
Lỗi tham số: tra get_tool_definition, sửa đúng trường lỗi rồi thử lại; không lặp nguyên lời gọi đã bị từ chối. Lỗi nghiệp vụ hoặc dữ kiện mâu thuẫn: nêu ngắn chỗ cần quyết định và hỏi làm rõ; không tự thay ý người dùng. Nếu vẫn không lưu được, nói rõ phần chưa lưu, không hỏi lại thông tin đã biết.
Khớp tên với context để lấy code; nhiều ứng viên hoặc tham chiếu tương đối chưa rõ thì hỏi lại. Kết quả tra cứu chưa đầy đủ không chứng minh thực thể chưa tồn tại. Không dùng code từ ví dụ làm định danh thật.';


-- -----------------------------------------------------------------------------
-- 1. System Genres
-- -----------------------------------------------------------------------------
-- Danh mục tag độc lập; mô tả xác định phạm vi của từng tag.
INSERT INTO genres (id, code, display_name, description) VALUES
('01a1c069-c9f1-7042-9dce-c80bdecf6301', 'xianxia', 'Tiên Hiệp', 'Tiểu thuyết tu chân lấy cảm hứng từ Đạo giáo, Phật giáo và thần thoại Á Đông, tập trung vào hành trình tu luyện nghịch thiên cầu trường sinh, luyện đan luyện khí, độ kiếp phi thăng và lĩnh ngộ thiên địa đại đạo.'),
('01a1c069-c9ee-75e0-9f59-0c4f0c2ff330', 'xuanhuan', 'Huyền Huyễn', 'Tiểu thuyết kỳ ảo phương Đông xây dựng trên nền tảng siêu hình học, tu luyện đấu khí/hồn lực, huyết mạch thần thú và đa tầng vũ trụ, kết hợp tự do các yếu tố thần thoại.'),
('01a1c069-c9f0-7d92-8bab-b56f5f114d46', 'wuxia', 'Kiếm hiệp', 'Tiểu thuyết võ thuật giang hồ và võ lâm Trung Quốc, Hàn Quốc (Murim), xoay quanh môn phái, bí kíp, ân oán chính tà và tinh thần hiệp nghĩa.'),
('01a1c069-c9ff-7ec0-a358-cc9d23d849d7', 'gaowu', 'Cao Võ', 'Thể loại giao thoa nơi võ đạo phát triển đến mức phi phàm dời non lấp bể, phá toái hư không, thường đặt trong bối cảnh toàn dân luyện võ, đô thị tương lai hoặc tinh tế.'),
('01a1c069-c9f2-730d-bc48-eceefe66f172', 'urban_dushi', 'Đô Thị', 'Tag bối cảnh thành thị với đời sống, thiết chế và quan hệ xã hội đô thị; có thể kết hợp với thời đại, kỳ ảo, dị năng hoặc lãng mạn.'),
('01a1c069-c9f4-7bed-b5a1-87aef9bd1895', 'military_junshi', 'Quân Sự', 'Tiểu thuyết chiến tranh, quân đội, chiến dịch quân sự, đặc chủng binh vương và chiến thuật sa trường với tinh thần đồng đội và lòng trung kiên.'),
('01a1c069-c9f9-7d6a-acd6-1247dc538e03', 'supernatural_lingyi', 'Linh Dị', 'Tiểu thuyết huyền bí tâm linh, trừ ma diệt quỷ, phong thủy bí thuật, trinh thám tâm linh và các quy tắc thế giới quỷ dị siêu nhiên.'),
('01a1c069-ca0e-7dd5-ace6-bd991c3f32c0', 'fan_fiction', 'Đồng Nhân', 'Tác phẩm phóng tác viết dựa trên thế giới quan, bối cảnh và tuyến nhân vật sẵn có từ các bộ anime, manga, game, tiểu thuyết hoặc phim ảnh nổi tiếng.'),
('01a1c069-ca56-7d71-a789-f499936dd3b6', 'mat_the', 'Mạt Thế', 'Tiểu thuyết sinh tồn khi xã hội sụp đổ trước đại dịch zombie, biến dị sinh học, thiên tai diệt vong hoặc dị tộc xâm lăng.'),
('01a1c069-ca57-7d54-805b-9344074fe045', 'he_thong', 'Hệ Thống', 'Tiểu thuyết nơi nhân vật sở hữu bàn tay vàng là giao diện thông minh giao nhiệm vụ, tích lũy điểm thưởng, quay thưởng gacha và thăng cấp chỉ số thần tốc.'),
('01a1c069-ca54-7c75-989b-2be9f3172c2b', 'xuyen_khong', 'Xuyên không', 'Thể loại linh hồn hoặc thể xác vượt qua không gian, thời gian sang thế giới khác, thời đại khác hoặc nhập vào nhân vật trong sách.'),
('01a1c069-ca3e-7b16-952b-71bebd7f0e78', 'trong_sinh', 'Trọng Sinh', 'Nhân vật sau khi trải qua bi kịch hoặc cái chết được quay ngược thời gian về quá khứ với toàn bộ ký ức để sửa chữa sai lầm và thay đổi số phận.'),
('01a1c069-ca52-7c5d-af48-07f1de9b3320', 'di_gioi', 'Dị Giới', 'Tiểu thuyết diễn ra tại một thế giới hoàn toàn khác biệt với Trái Đất, nơi tồn tại các quy luật ma thuật, đấu khí và các chủng tộc kỳ ảo.'),
('01a1c069-ca53-771d-b479-3ec33bdf45f6', 'di_nang', 'Dị Năng', 'Tiểu thuyết về cá nhân thức tỉnh siêu năng lực, học cách kiểm soát sức mạnh và đối mặt xung đột xã hội; bao gồm các câu chuyện siêu anh hùng.'),
('01a1c069-ca00-7604-bd37-fa2f5519bd4a', 'danmei', 'Đam Mỹ', 'Tiểu thuyết tình cảm lãng mạn giữa nam và nam (Boys Love / BL) trải rộng từ tiên hiệp, cổ trang, hiện đại cho đến vô hạn lưu và tinh tế.'),
('01a1c069-ca06-7e62-9924-3ebebb88d095', 'baihe', 'Bách Hợp', 'Tiểu thuyết tình cảm lãng mạn giữa nữ và nữ (Girls Love / GL) trong các bối cảnh kỳ ảo, cổ đại hoặc đời thường.'),
('01a1c069-ca04-79b4-a317-f936df64615d', 'vo_han_luu', 'Vô Hạn Lưu', 'Thể loại nơi nhân vật bị kéo vào chiều không gian kỳ bí, liên tục vượt qua các phó bản sinh tồn, giải mã quy tắc tử thần để đổi lấy tích điểm sống còn.'),
('01a1c069-ca30-7a30-8d6c-2d7ef19d392f', 'dien_van', 'Điền Văn & Trồng Trọt', 'Tiểu thuyết nhịp điệu êm đềm tập trung vào đời sống gia đình, làm nông nghiệp, phát triển sản nghiệp, kinh tế gia đình từ hai bàn tay trắng.'),
('01a1c069-ca3d-70c5-926b-58ceffc8ba41', 'hunter_dungeon', 'Hầm Ngục, Thợ Săn', 'Tiểu thuyết về thợ săn thức tỉnh, cổng không gian, chinh phục hầm ngục hoặc tháp thử thách; có thể khai thác chòm sao bảo hộ hay nhân vật điều khiển lõi hầm ngục.'),
('01a1c069-ca1a-7b61-b866-02a46cd502e6', 'shounen', 'Shounen', 'Tác phẩm hướng tới thanh thiếu niên, nhấn mạnh tinh thần đồng đội nỗ lực, tình bạn bền chặt, phiêu lưu vượt thử thách và chiến đấu hết mình.'),
('01a1c069-ca19-7d2d-8f44-a9a115544043', 'seinen', 'Seinen', 'Tác phẩm hướng tới độc giả nam trưởng thành, cốt truyện phức tạp, chiều sâu tâm lý, chính trị hoặc yếu tố hành động chân thực tàn khốc hơn.'),
('01a1c069-ca1b-7d70-9fc7-1b76343f77bb', 'shoujo', 'Shoujo', 'Tác phẩm hướng tới độc giả nữ trẻ tuổi, tập trung vào cảm xúc nội tâm, nét vẽ mộng mơ, rung động tình đầu và các mối quan hệ học đường.'),
('01a1c069-ca1c-71b9-9e57-d8bd67cabf33', 'josei', 'Josei', 'Tác phẩm hướng tới độc giả nữ trưởng thành, miêu tả các vấn đề tình yêu, hôn nhân, gia đình và sự nghiệp dưới góc nhìn chín chắn, thực tế.'),
('01a1c069-ca20-7b16-98fe-9958f05a604f', 'mecha', 'Mecha', 'Thể loại khoa học viễn tưởng xoay quanh các cỗ máy chiến đấu khổng lồ do con người điều khiển trong chiến tranh liên sao.'),
('01a1c069-ca21-7543-bc1f-29d9eb707f76', 'slice_of_life', 'Đời thường', 'Thể loại khắc họa chân thực, nhẹ nhàng những khoảnh khắc đời thường của nhân vật, không tập trung vào xung đột kịch tính lớn.'),
('01a1c069-ca22-7c62-bd6b-d1d96ee55c18', 'harem', 'Harem', 'Thể loại nơi một nhân vật chính được theo đuổi, vây quanh và yêu mến bởi nhiều nhân vật khác giới.'),
('01a1c069-ca11-7da3-8dba-eb84a7f898f3', 'litrpg', 'LitRPG', 'Tiểu thuyết tích hợp chỉ số, cấp độ, nhiệm vụ và cơ chế game vào thế giới truyện, bao gồm võng du, trò chơi trực tuyến thực tế ảo và game hóa hiện thực.'),
('01a1c069-ca5b-7fc3-952d-4e64724cd3f4', 'science_fiction', 'Khoa Học Viễn Tưởng', 'Tiểu thuyết suy đoán về khoa học, công nghệ, trí tuệ nhân tạo và tương lai nhân loại; bao gồm khoa huyễn Á Đông và công nghệ giả tưởng thời đại hơi nước.'),
('01a1c069-ca78-7af1-9c8f-5bf3b55740a8', 'space_opera', 'Space Opera', 'Tiểu thuyết viễn tưởng hoành tráng lấy bối cảnh chiến tranh giữa các đế chế thiên hà, phi thuyền chiến hạm và phiêu lưu đa hành tinh.'),
('01a1c069-ca73-7753-823e-c47097179aa1', 'cyberpunk', 'Cyberpunk', 'Khoa học viễn tưởng tương lai phản địa đàng, nơi công nghệ AI, mạng kết nối và cấy ghép sinh học thống trị bên cạnh sự suy đồi xã hội.'),
('01a1c069-ca5c-75e9-b380-a9824f778356', 'dystopian', 'Phản Địa Đàng', 'Tiểu thuyết lấy bối cảnh xã hội tương lai bị kiểm soát toàn trị, áp bức khắc nghiệt nơi cá nhân vùng lên đòi lại tự do.'),
('01a1c069-ca71-7bce-9c55-9d57b1bebacd', 'alternate_history', 'Alternate History', 'Tiểu thuyết suy đoán điều gì sẽ xảy ra nếu một biến cố lịch sử quan trọng trong quá khứ diễn biến theo chiều hướng ngược lại.'),
('01a1c069-ca5e-70fe-a423-674439619334', 'mystery_detective', 'Trinh thám', 'Tiểu thuyết xoay quanh quá trình điều tra phá án, truy lùng hung thủ của thám tử chuyên nghiệp hoặc nghiệp dư.'),
('01a1c069-ca39-7f7b-b4bf-02552e9a6c2b', 'psychological', 'Tâm Lý', 'Tiểu thuyết đi sâu vào các ngóc ngách tinh thần, sự thao túng tâm lý, chấn thương tiềm thức và nhân cách con người.'),
('01a1c069-ca5f-7739-94a2-ce2531c64875', 'horror', 'Horror', 'Tiểu thuyết khơi gợi cảm giác sợ hãi nguyên thủy trước quái vật, thế lực tà ác siêu nhiên hoặc sự điên loạn tâm thần.'),
('01a1c069-ca65-7144-af38-f78b9746ff74', 'contemporary_fiction', 'Contemporary Fiction', 'Tác phẩm hư cấu phản ánh trực tiếp đời sống, mâu thuẫn xã hội và tâm tư con người trong thế giới hôm nay.'),
('01a1c069-ca66-712f-8229-8c805ff7429f', 'literary_fiction', 'Literary Fiction', 'Tiểu thuyết chú trọng phong cách ngôn từ nghệ thuật, chiều sâu tư tưởng và nhân văn hơn các công thức giải trí.'),
('01a1c069-ca0a-7005-b755-89e4d6b45d16', 'magical_realism', 'Magical Realism (Hiện Thực Huyền Ảo)', 'Tiểu thuyết khắc họa hiện thực đời sống nhưng đan cài các hiện tượng kỳ ảo như một phần tự nhiên của thế giới.'),
('01a1c069-c9f7-752a-9b1e-acd95edd8d84', 'sports', 'Thể Thao Chuyên Nghiệp', 'Tiểu thuyết về đấu trường thể thao, tinh thần rèn luyện và vinh quang đỉnh cao của vận động viên.'),
('019a6f00-0000-7000-8000-000000000001', 'comedy', 'Hài Hước', 'Tác phẩm lấy tiếng cười làm trọng tâm qua tình huống trớ trêu, đối thoại dí dỏm, châm biếm hoặc phi lý; có thể kết hợp với các thể loại khác.'),
('019a6f01-0000-7000-8000-000000000001', 'romance', 'Lãng Mạn', 'Tác phẩm lấy sự hình thành và phát triển tình yêu làm tuyến chính, độc lập với thời đại và bối cảnh.'),
('019a6f01-0000-7000-8000-000000000002', 'fantasy', 'Kỳ Ảo', 'Tác phẩm có ma thuật, thần thoại hoặc quy luật kỳ ảo giữ vai trò quan trọng; không quy định thời đại, địa điểm, mức độ phép thuật hay tuyến tình cảm.'),
('019a6f01-0000-7000-8000-000000000003', 'ancient', 'Cổ Đại', 'Tag bối cảnh xã hội cổ đại hoặc cổ phong hư cấu; không mặc định có tình yêu hay dựa trên một giai đoạn lịch sử có thật.'),
('019a6f01-0000-7000-8000-000000000004', 'modern', 'Hiện Đại', 'Tag bối cảnh thời hiện đại, có thể ở thành thị hoặc nông thôn; không mặc định tuyến tình cảm.'),
('019a6f01-0000-7000-8000-000000000005', 'academy', 'Học Viện', 'Tag bối cảnh trường học hoặc cơ sở đào tạo, gồm đời sống học tập và quan hệ thầy trò; không mặc định ma thuật.'),
('019a6f01-0000-7000-8000-000000000006', 'dark', 'Đen Tối', 'Tag sắc thái u ám, tàn khốc hoặc đạo đức xám; không mặc định nhân vật phải hắc hóa, có yếu tố kinh dị hay quan hệ tình cảm độc hại.'),
('019a6f01-0000-7000-8000-000000000007', 'supernatural', 'Siêu Nhiên', 'Tag về thực thể hoặc hiện tượng vượt quy luật tự nhiên như linh hồn, ma cà rồng và người sói; không mặc định gây sợ hãi hoặc có tình yêu.'),
('019a6f01-0000-7000-8000-000000000008', 'progression', 'Thăng Tiến Năng Lực', 'Tag về trọng tâm rèn luyện và tăng năng lực qua các cấp độ có thể theo dõi; không bắt buộc ma thuật, hệ thống game, xuất phát điểm phế vật hay báo thù.'),
('019a6f01-0000-7000-8000-000000000009', 'action', 'Hành Động', 'Tác phẩm lấy các hành động đối đầu, chiến đấu, truy đuổi và ứng phó hiểm nguy làm trọng tâm.'),
('019a6f01-0000-7000-8000-00000000000a', 'adventure', 'Phiêu Lưu', 'Tác phẩm lấy hành trình khám phá, thám hiểm và trải nghiệm ở những vùng đất hoặc hoàn cảnh mới làm trọng tâm.')
ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name, description = EXCLUDED.description;

DELETE FROM genres WHERE code IN ("sci_fi", "mystery");

-- -----------------------------------------------------------------------------
-- 2. System Motifs & Associated Structural Components
-- -----------------------------------------------------------------------------
-- Tham chiếu nghiên cứu thị trường: + [Reedsy - Hero's Journey: Get a Strong Story Structure in 12 Steps](https://reedsy.com/blog/guide/story-structure/heros-journey/):
-- 1. Table motifs
INSERT INTO motifs (id, code, display_name, category, version, is_system, description, min_chapter, max_chapter) VALUES
  ('018f0001-0000-7000-8000-000000000001', 'heros_journey_12_steps', 'Hành trình Anh hùng 12 Giai đoạn (The Hero''s Journey)', 'plot', '1.0.0', true, 'Cấu trúc kinh điển 12 giai đoạn kế thừa từ Joseph Campbell và Christopher Vogler, dẫn dắt nhân vật từ Thế giới bình thường qua ngọn lửa thử thách và trở về với Thần dược biến đổi.', 24, 120);

-- 2. Table motif_applicable_genres
INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f0001-0000-7000-8000-000000000015', '018f0001-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'fantasy'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f0001-0000-7000-8000-000000000016', '018f0001-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'action'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f0001-0000-7000-8000-000000002016', '018f0001-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'adventure'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f0001-0000-7000-8000-000000000017', '018f0001-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'space_opera'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

-- 3. Table state_rules
INSERT INTO state_rules (id, code, display_name) VALUES
  ('018f0001-0000-7000-8000-000000000029', 'linear_stage_progression', 'Quy tắc tiến trình tuyến tính tuần tự')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO state_rules (id, code, display_name) VALUES
  ('018f0001-0000-7000-8000-00000000002a', 'threshold_point_of_no_return', 'Quy tắc vượt ngưỡng không thể quay đầu')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

-- 4. Table states & state_down_by_rules
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0001-0000-7000-8000-000000000065', 'hj_ordinary_world', 'Thế giới Bình thường (The Ordinary World)', 'Nhân vật xuất phát điểm từ cuộc sống đơn điệu, ẩn chứa cảm giác thiếu sót hoặc bất an.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0001-0000-7000-8000-0000000000d2', '018f0001-0000-7000-8000-000000000065', (SELECT id FROM state_rules WHERE code = 'linear_stage_progression'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0001-0000-7000-8000-000000000066', 'hj_call_to_adventure', 'Tiếng gọi Lên đường (Call to Adventure)', 'Sự kiện phá vỡ thế cân bằng, đặt ra mục tiêu hoặc nguy cơ đe dọa.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0001-0000-7000-8000-0000000000dc', '018f0001-0000-7000-8000-000000000066', (SELECT id FROM state_rules WHERE code = 'linear_stage_progression'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0001-0000-7000-8000-000000000067', 'hj_refusal_of_call', 'Từ chối Tiếng gọi (Refusal of the Call)', 'Sự do dự, sợ hãi, ràng buộc gia đình hoặc gánh nặng quá khứ kìm hãm nhân vật.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0001-0000-7000-8000-0000000000e6', '018f0001-0000-7000-8000-000000000067', (SELECT id FROM state_rules WHERE code = 'linear_stage_progression'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0001-0000-7000-8000-000000000068', 'hj_meeting_mentor', 'Gặp gỡ Người dẫn đường (Meeting the Mentor)', 'Nhận được sự chỉ bảo, báu vật hoặc niềm tin từ người thầy thông thái.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0001-0000-7000-8000-0000000000f0', '018f0001-0000-7000-8000-000000000068', (SELECT id FROM state_rules WHERE code = 'linear_stage_progression'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0001-0000-7000-8000-000000000069', 'hj_crossing_first_threshold', 'Vượt qua Ngưỡng cửa Đầu tiên (Crossing the First Threshold)', 'Chính thức bước vào Thế giới Đặc biệt đầy hiểm họa và phép thuật.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0001-0000-7000-8000-0000000000fb', '018f0001-0000-7000-8000-000000000069', (SELECT id FROM state_rules WHERE code = 'threshold_point_of_no_return'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0001-0000-7000-8000-00000000006a', 'hj_tests_allies_enemies', 'Thử thách, Đồng minh và Kẻ thù (Tests, Allies, Enemies)', 'Làm quen với luật chơi mới, xây dựng liên minh và đối phó với tay chân phản diện.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0001-0000-7000-8000-000000000104', '018f0001-0000-7000-8000-00000000006a', (SELECT id FROM state_rules WHERE code = 'linear_stage_progression'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0001-0000-7000-8000-00000000006b', 'hj_approach_inmost_cave', 'Tiếp cận Hang sâu Tận cùng (Approach to the Inmost Cave)', 'Chuẩn bị tâm thế và nguồn lực trước khi chạm trán trung tâm hiểm nguy.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0001-0000-7000-8000-00000000010e', '018f0001-0000-7000-8000-00000000006b', (SELECT id FROM state_rules WHERE code = 'linear_stage_progression'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0001-0000-7000-8000-00000000006c', 'hj_the_ordeal', 'Đại Thử thách Sinh tử (The Ordeal)', 'Đối diện với cái chết, chạm đáy khủng hoảng để tái sinh với sức mạnh mới.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0001-0000-7000-8000-000000000118', '018f0001-0000-7000-8000-00000000006c', (SELECT id FROM state_rules WHERE code = 'linear_stage_progression'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0001-0000-7000-8000-00000000006d', 'hj_reward_seizing_sword', 'Phần thưởng / Đoạt lấy Bảo kiếm (Reward)', 'Chiến thắng thử thách cốt lõi, nắm giữ thần vật hoặc tri thức giác ngộ.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0001-0000-7000-8000-000000000122', '018f0001-0000-7000-8000-00000000006d', (SELECT id FROM state_rules WHERE code = 'linear_stage_progression'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0001-0000-7000-8000-00000000006e', 'hj_the_road_back', 'Đường về Đầy Sóng gió (The Road Back)', 'Sự truy đuổi điên cuồng của phe ác, hậu quả của việc mang bảo vật trở về.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0001-0000-7000-8000-00000000012c', '018f0001-0000-7000-8000-00000000006e', (SELECT id FROM state_rules WHERE code = 'linear_stage_progression'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0001-0000-7000-8000-00000000006f', 'hj_resurrection', 'Sự Phục sinh / Khảo nghiệm Cuối cùng (The Resurrection)', 'Trận chiến tối thượng thử nghiệm sự biến đổi toàn diện của người anh hùng.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0001-0000-7000-8000-000000000136', '018f0001-0000-7000-8000-00000000006f', (SELECT id FROM state_rules WHERE code = 'linear_stage_progression'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0001-0000-7000-8000-000000000070', 'hj_return_with_elixir', 'Trở về cùng Thần dược (Return with the Elixir)', 'Mang theo bình an, tri thức cứu rỗi cộng đồng và hoàn thiện bản thân.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0001-0000-7000-8000-000000000140', '018f0001-0000-7000-8000-000000000070', (SELECT id FROM state_rules WHERE code = 'linear_stage_progression'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

-- 5. Table components
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0001-0000-7000-8000-00000000012d', 'cmp_status_quo_setup', 'Thiết lập Hiện trạng', 'Khắc họa khát vọng ngầm và bối cảnh sinh sống.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0001-0000-7000-8000-00000000012e', 'cmp_herald_arrival', 'Sứ giả Báo hiệu', 'Mang đến thông điệp hoặc lời thách thức định mệnh.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0001-0000-7000-8000-00000000012f', 'cmp_hesitation_crisis', 'Khủng hoảng Do dự', 'Nỗi sợ thất bại đối đầu với áp lực nội tại.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0001-0000-7000-8000-000000000130', 'cmp_mentor_bestowal', 'Tôn sư Ban ấn', 'Vũ khí đặc biệt hoặc kim chỉ nam tinh thần.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0001-0000-7000-8000-000000000131', 'cmp_gatekeeper_defeat', 'Hạ Người giữ cổng', 'Hành vi quyết đoán bước qua điểm không thể quay lại.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0001-0000-7000-8000-000000000132', 'cmp_trials_gauntlet', 'Chuỗi Thử thách Tôi rèn', 'Tập hợp đồng đội trung thành và phân hóa kẻ thù.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0001-0000-7000-8000-000000000133', 'cmp_inmost_advance', 'Tiến công Sào huyệt', 'Thâm nhập hang ổ rồng hoặc lãnh địa bóng tối.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0001-0000-7000-8000-000000000134', 'cmp_death_brush', 'Trải nghiệm Cận tử', 'Hiến tế cái tôi cũ để phá vỡ giới hạn phàm trần.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0001-0000-7000-8000-000000000135', 'cmp_treasure_claim', 'Chiếm đoạt Bảo vật', 'Thu hoạch quả ngọt của lòng dũng cảm.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0001-0000-7000-8000-000000000136', 'cmp_perilous_escape', 'Thoát hiểm Nguy nan', 'Cuộc rượt đuổi tốc độ cao về lại ranh giới quen thuộc.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0001-0000-7000-8000-000000000137', 'cmp_final_cleansing', 'Thanh tẩy Tối hậu', 'Chiến thắng quyết định xóa tan tàn dư ác mộng.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0001-0000-7000-8000-000000000138', 'cmp_boon_distribution', 'Phân phát Phúc lành', 'Chữa lành quê hương và thiết lập trật tự tươi sáng.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

-- 6. Table emotional_ranges
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0001-0000-7000-8000-000000000191', 'emo_monotony', 'Đơn điệu Ngột ngạt')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0001-0000-7000-8000-000000000192', 'emo_shock_disruption', 'Choáng váng Chấn động')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0001-0000-7000-8000-000000000193', 'emo_apprehension', 'Bồn chồn Lo sợ')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0001-0000-7000-8000-000000000194', 'emo_reassurance', 'An tâm Tin tưởng')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0001-0000-7000-8000-000000000195', 'emo_solemn_resolve', 'Quyết tâm Trang nghiêm')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0001-0000-7000-8000-000000000196', 'emo_camaraderie', 'Tình bạn Đồng cam Cộng khổ')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0001-0000-7000-8000-000000000197', 'emo_suffocating_dread', 'Nỗi kinh hoàng Nghẹt thở')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0001-0000-7000-8000-000000000198', 'emo_mortal_catharsis', 'Giải thoát Sinh tử')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0001-0000-7000-8000-000000000199', 'emo_exultation', 'Hân hoan Chiến thắng')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0001-0000-7000-8000-00000000019a', 'emo_racing_urgency', 'Gấp gáp Nguy cấp')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0001-0000-7000-8000-00000000019b', 'emo_sublime_triumph', 'Chiến thắng Siêu phàm')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0001-0000-7000-8000-00000000019c', 'emo_transcendent_peace', 'Bình yên Thanh thản')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

-- 7. Table event_blocks & event_block_emotionals
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0001-0000-7000-8000-0000000001f5', 'eb_hj_ordinary_world', '018f0001-0000-7000-8000-00000000012d', '018f0001-0000-7000-8000-000000000065', '018f0001-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0001-0000-7000-8000-000000000262', (SELECT id FROM emotional_ranges WHERE code = 'emo_monotony'), '018f0001-0000-7000-8000-0000000001f5')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0001-0000-7000-8000-0000000001f6', 'eb_hj_call', '018f0001-0000-7000-8000-00000000012e', '018f0001-0000-7000-8000-000000000066', '018f0001-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0001-0000-7000-8000-00000000026d', (SELECT id FROM emotional_ranges WHERE code = 'emo_shock_disruption'), '018f0001-0000-7000-8000-0000000001f6')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0001-0000-7000-8000-0000000001f7', 'eb_hj_refusal', '018f0001-0000-7000-8000-00000000012f', '018f0001-0000-7000-8000-000000000067', '018f0001-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0001-0000-7000-8000-000000000278', (SELECT id FROM emotional_ranges WHERE code = 'emo_apprehension'), '018f0001-0000-7000-8000-0000000001f7')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0001-0000-7000-8000-0000000001f8', 'eb_hj_mentor', '018f0001-0000-7000-8000-000000000130', '018f0001-0000-7000-8000-000000000068', '018f0001-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0001-0000-7000-8000-000000000283', (SELECT id FROM emotional_ranges WHERE code = 'emo_reassurance'), '018f0001-0000-7000-8000-0000000001f8')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0001-0000-7000-8000-0000000001f9', 'eb_hj_threshold', '018f0001-0000-7000-8000-000000000131', '018f0001-0000-7000-8000-000000000069', '018f0001-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0001-0000-7000-8000-00000000028e', (SELECT id FROM emotional_ranges WHERE code = 'emo_solemn_resolve'), '018f0001-0000-7000-8000-0000000001f9')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0001-0000-7000-8000-0000000001fa', 'eb_hj_trials', '018f0001-0000-7000-8000-000000000132', '018f0001-0000-7000-8000-00000000006a', '018f0001-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0001-0000-7000-8000-000000000299', (SELECT id FROM emotional_ranges WHERE code = 'emo_camaraderie'), '018f0001-0000-7000-8000-0000000001fa')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0001-0000-7000-8000-0000000001fb', 'eb_hj_approach', '018f0001-0000-7000-8000-000000000133', '018f0001-0000-7000-8000-00000000006b', '018f0001-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0001-0000-7000-8000-0000000002a4', (SELECT id FROM emotional_ranges WHERE code = 'emo_suffocating_dread'), '018f0001-0000-7000-8000-0000000001fb')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0001-0000-7000-8000-0000000001fc', 'eb_hj_ordeal', '018f0001-0000-7000-8000-000000000134', '018f0001-0000-7000-8000-00000000006c', '018f0001-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0001-0000-7000-8000-0000000002af', (SELECT id FROM emotional_ranges WHERE code = 'emo_mortal_catharsis'), '018f0001-0000-7000-8000-0000000001fc')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0001-0000-7000-8000-0000000001fd', 'eb_hj_reward', '018f0001-0000-7000-8000-000000000135', '018f0001-0000-7000-8000-00000000006d', '018f0001-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0001-0000-7000-8000-0000000002ba', (SELECT id FROM emotional_ranges WHERE code = 'emo_exultation'), '018f0001-0000-7000-8000-0000000001fd')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0001-0000-7000-8000-0000000001fe', 'eb_hj_road_back', '018f0001-0000-7000-8000-000000000136', '018f0001-0000-7000-8000-00000000006e', '018f0001-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0001-0000-7000-8000-0000000002c5', (SELECT id FROM emotional_ranges WHERE code = 'emo_racing_urgency'), '018f0001-0000-7000-8000-0000000001fe')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0001-0000-7000-8000-0000000001ff', 'eb_hj_resurrection', '018f0001-0000-7000-8000-000000000137', '018f0001-0000-7000-8000-00000000006f', '018f0001-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0001-0000-7000-8000-0000000002d0', (SELECT id FROM emotional_ranges WHERE code = 'emo_sublime_triumph'), '018f0001-0000-7000-8000-0000000001ff')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0001-0000-7000-8000-000000000200', 'eb_hj_return', '018f0001-0000-7000-8000-000000000138', '018f0001-0000-7000-8000-000000000070', '018f0001-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0001-0000-7000-8000-0000000002db', (SELECT id FROM emotional_ranges WHERE code = 'emo_transcendent_peace'), '018f0001-0000-7000-8000-000000000200')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

-- 8. Table event_transitions
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0001-0000-7000-8000-0000000002bd', 'Bước qua Tiếng gọi', '018f0001-0000-7000-8000-0000000001f5', '018f0001-0000-7000-8000-0000000001f6', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0001-0000-7000-8000-0000000002be', 'Rút lui do Do dự', '018f0001-0000-7000-8000-0000000001f6', '018f0001-0000-7000-8000-0000000001f7', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0001-0000-7000-8000-0000000002bf', 'Thức tỉnh nhờ Tiền bối', '018f0001-0000-7000-8000-0000000001f7', '018f0001-0000-7000-8000-0000000001f8', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0001-0000-7000-8000-0000000002c0', 'Vượt ranh giới Thực tại', '018f0001-0000-7000-8000-0000000001f8', '018f0001-0000-7000-8000-0000000001f9', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0001-0000-7000-8000-0000000002c1', 'Đối đầu Sa trường', '018f0001-0000-7000-8000-0000000001f9', '018f0001-0000-7000-8000-0000000001fa', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0001-0000-7000-8000-0000000002c2', 'Thâm nhập Nguyệt động', '018f0001-0000-7000-8000-0000000001fa', '018f0001-0000-7000-8000-0000000001fb', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0001-0000-7000-8000-0000000002c3', 'Chạm trán Vực thẳm Hủy diệt', '018f0001-0000-7000-8000-0000000001fb', '018f0001-0000-7000-8000-0000000001fc', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0001-0000-7000-8000-0000000002c4', 'Đoạt Chân mệnh Thánh vật', '018f0001-0000-7000-8000-0000000001fc', '018f0001-0000-7000-8000-0000000001fd', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0001-0000-7000-8000-0000000002c5', 'Trở về trong Khói lửa', '018f0001-0000-7000-8000-0000000001fd', '018f0001-0000-7000-8000-0000000001fe', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0001-0000-7000-8000-0000000002c6', 'Khảo nghiệm Sinh tử Tối cao', '018f0001-0000-7000-8000-0000000001fe', '018f0001-0000-7000-8000-0000000001ff', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0001-0000-7000-8000-0000000002c7', 'Bình định Thế giới', '018f0001-0000-7000-8000-0000000001ff', '018f0001-0000-7000-8000-000000000200', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

-- 9. Table constraints & motif_constraints
INSERT INTO constraints (id, code, require, reason) VALUES
  ('018f0001-0000-7000-8000-000000000321', 'hj_mandatory_ordeal', 'Giai đoạn Đại thử thách (The Ordeal) bắt buộc phải diễn ra trước khi nhân vật nhận phần thưởng.', 'Người anh hùng không thể trưởng thành về mặt tâm thức nếu không vượt qua trải nghiệm cận tử.')
  ON CONFLICT (code) DO UPDATE SET require = EXCLUDED.require, reason = EXCLUDED.reason;

INSERT INTO motif_constraints (id, code, constraint_id, motif_id, syntax, constrain) VALUES
  ('018f0001-0000-7000-8000-000000000385', 'heros_journey_12_steps_hj_mandatory_ordeal', (SELECT id FROM constraints WHERE code = 'hj_mandatory_ordeal'), '018f0001-0000-7000-8000-000000000001', 'VALIDATE_MOTIF_RULE(''heros_journey_12_steps'', ''hj_mandatory_ordeal'')', 'Giai đoạn Đại thử thách (The Ordeal) bắt buộc phải diễn ra trước khi nhân vật nhận phần thưởng.')
  ON CONFLICT (code) DO UPDATE SET syntax = EXCLUDED.syntax, constrain = EXCLUDED.constrain;

INSERT INTO constraints (id, code, require, reason) VALUES
  ('018f0001-0000-7000-8000-000000000322', 'hj_elixir_restoration', 'Thần dược (Elixir) phải có khả năng chữa lành hoặc thăng hoa thế giới xuất phát điểm.', 'Bảo đảm tính chu toàn của vòng cung hành trình phụng sự cộng đồng.')
  ON CONFLICT (code) DO UPDATE SET require = EXCLUDED.require, reason = EXCLUDED.reason;

INSERT INTO motif_constraints (id, code, constraint_id, motif_id, syntax, constrain) VALUES
  ('018f0001-0000-7000-8000-000000000386', 'heros_journey_12_steps_hj_elixir_restoration', (SELECT id FROM constraints WHERE code = 'hj_elixir_restoration'), '018f0001-0000-7000-8000-000000000001', 'VALIDATE_MOTIF_RULE(''heros_journey_12_steps'', ''hj_elixir_restoration'')', 'Thần dược (Elixir) phải có khả năng chữa lành hoặc thăng hoa thế giới xuất phát điểm.')
  ON CONFLICT (code) DO UPDATE SET syntax = EXCLUDED.syntax, constrain = EXCLUDED.constrain;

-- Tham chiếu nghiên cứu thị trường: + [Save the Cat! - What Is the Save the Cat Method? 15 Beat Sheet Guide](https://savethecat.com/get-started):
-- 1. Table motifs
INSERT INTO motifs (id, code, display_name, category, version, is_system, description, min_chapter, max_chapter) VALUES
  ('018f0002-0000-7000-8000-000000000001', 'save_the_cat_15_beats', 'Cấu trúc 15 Nhịp Save the Cat (Blake Snyder Beat Sheet)', 'plot', '1.0.0', true, 'Phương pháp tự sự điện ảnh 15 nhịp phân chia thành 3 Hồi với tỷ lệ nhịp điệu chính xác: từ Opening Image, Catalyst, Break into Two, Midpoint, All is Lost đến Final Image.', 20, 60);

-- 2. Table motif_applicable_genres
INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f0002-0000-7000-8000-000000000015', '018f0002-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'contemporary_fiction'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

-- 3. Table state_rules
INSERT INTO state_rules (id, code, display_name) VALUES
  ('018f0002-0000-7000-8000-000000000029', 'three_act_percentage_rule', 'Quy tắc tỷ lệ phân bổ 3 Hồi chuẩn xác')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO state_rules (id, code, display_name) VALUES
  ('018f0002-0000-7000-8000-00000000002a', 'midpoint_polarity_reversal', 'Quy tắc đảo chiều cực tính tại Điểm giữa')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

-- 4. Table states & state_down_by_rules
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0002-0000-7000-8000-000000000065', 'stc_opening_image', 'Hình ảnh Mở đầu (Opening Image)', 'Bức tranh chụp nhanh về thế giới của nhân vật trước khi hành trình bắt đầu.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0002-0000-7000-8000-0000000000d2', '018f0002-0000-7000-8000-000000000065', (SELECT id FROM state_rules WHERE code = 'three_act_percentage_rule'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0002-0000-7000-8000-000000000066', 'stc_set_up', 'Thiết lập Bối cảnh & Thiếu sót (Set-Up)', 'Giới thiệu nhân vật chính, các khiếm khuyết nội tâm (Six Things That Need Fixing).')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0002-0000-7000-8000-0000000000dc', '018f0002-0000-7000-8000-000000000066', (SELECT id FROM state_rules WHERE code = 'three_act_percentage_rule'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0002-0000-7000-8000-000000000067', 'stc_catalyst', 'Chất xúc tác (Catalyst)', 'Cú hích chấn động phá hủy thế giới cũ, mở ra nhiệm vụ mới.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0002-0000-7000-8000-0000000000e6', '018f0002-0000-7000-8000-000000000067', (SELECT id FROM state_rules WHERE code = 'three_act_percentage_rule'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0002-0000-7000-8000-000000000068', 'stc_debate', 'Tranh luận / Do dự (Debate)', 'Nhân vật băn khoăn liệu có nên chấp nhận thử thách.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0002-0000-7000-8000-0000000000f0', '018f0002-0000-7000-8000-000000000068', (SELECT id FROM state_rules WHERE code = 'three_act_percentage_rule'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0002-0000-7000-8000-000000000069', 'stc_break_into_two', 'Bước sang Hồi 2 (Break into Two)', 'Nhân vật chính chủ động lựa chọn rời bỏ thế giới cũ bước vào thế giới ngược lại.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0002-0000-7000-8000-0000000000fa', '018f0002-0000-7000-8000-000000000069', (SELECT id FROM state_rules WHERE code = 'three_act_percentage_rule'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0002-0000-7000-8000-00000000006a', 'stc_b_story', 'Tuyến truyện B / Mối quan hệ (B Story)', 'Nhân vật hỗ trợ xuất hiện, đại diện cho thông điệp chủ đề và chân lý.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0002-0000-7000-8000-000000000104', '018f0002-0000-7000-8000-00000000006a', (SELECT id FROM state_rules WHERE code = 'three_act_percentage_rule'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0002-0000-7000-8000-00000000006b', 'stc_fun_and_games', 'Hứng khởi & Thử nghiệm (Fun and Games)', 'Lời hứa của tiền đề câu chuyện (Promise of the Premise) thể hiện rõ nét nhất.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0002-0000-7000-8000-00000000010e', '018f0002-0000-7000-8000-00000000006b', (SELECT id FROM state_rules WHERE code = 'three_act_percentage_rule'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0002-0000-7000-8000-00000000006c', 'stc_midpoint', 'Điểm giữa (Midpoint)', 'Đỉnh cao chiến thắng ảo hoặc thất bại ảo; cọc cược được đẩy lên tối đa.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0002-0000-7000-8000-000000000119', '018f0002-0000-7000-8000-00000000006c', (SELECT id FROM state_rules WHERE code = 'midpoint_polarity_reversal'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0002-0000-7000-8000-00000000006d', 'stc_bad_guys_close_in', 'Kẻ xấu Xiết vòng vây (Bad Guys Close In)', 'Thế lực phản diện phản công, nghi kỵ nội bộ xuất hiện.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0002-0000-7000-8000-000000000122', '018f0002-0000-7000-8000-00000000006d', (SELECT id FROM state_rules WHERE code = 'three_act_percentage_rule'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0002-0000-7000-8000-00000000006e', 'stc_all_is_lost', 'Mất hết Tất cả (All Is Lost)', 'Đáy vực tuyệt vọng; cái chết tượng trưng hoặc cái chết của người thầy.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0002-0000-7000-8000-00000000012c', '018f0002-0000-7000-8000-00000000006e', (SELECT id FROM state_rules WHERE code = 'three_act_percentage_rule'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0002-0000-7000-8000-00000000006f', 'stc_dark_night_of_the_soul', 'Đêm tối Tâm hồn (Dark Night of the Soul)', 'Sự nghiền ngẫm đau đớn trước khi ngộ ra Chân lý thực sự.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0002-0000-7000-8000-000000000136', '018f0002-0000-7000-8000-00000000006f', (SELECT id FROM state_rules WHERE code = 'three_act_percentage_rule'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0002-0000-7000-8000-000000000070', 'stc_break_into_three', 'Bước sang Hồi 3 (Break into Three)', 'Tia sáng giác ngộ; tìm ra giải pháp mới kết hợp giữa mong muốn và nhu cầu.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0002-0000-7000-8000-000000000140', '018f0002-0000-7000-8000-000000000070', (SELECT id FROM state_rules WHERE code = 'three_act_percentage_rule'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0002-0000-7000-8000-000000000071', 'stc_finale', 'Đại kết cục (Finale)', 'Áp dụng bài học chân lý đánh bại kẻ thù và thanh lọc thế giới.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0002-0000-7000-8000-00000000014a', '018f0002-0000-7000-8000-000000000071', (SELECT id FROM state_rules WHERE code = 'three_act_percentage_rule'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0002-0000-7000-8000-000000000072', 'stc_final_image', 'Hình ảnh Kết thúc (Final Image)', 'Bức tranh đối chiếu trực tiếp với Opening Image, khẳng định sự lột xác vĩnh viễn.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0002-0000-7000-8000-000000000154', '018f0002-0000-7000-8000-000000000072', (SELECT id FROM state_rules WHERE code = 'three_act_percentage_rule'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

-- 5. Table components
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0002-0000-7000-8000-00000000012d', 'cmp_snap_shot', 'Khắc họa Điểm xuất phát', 'Bộc lộ điểm yếu chết người.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0002-0000-7000-8000-00000000012e', 'cmp_inciting_shocker', 'Sự cố Bất ngờ Châm ngòi', 'Tin báo tử hoặc mất việc làm.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0002-0000-7000-8000-00000000012f', 'cmp_deliberation_struggle', 'Đấu tranh Tư tưởng', 'Liệu có nên mạo hiểm dấn thân.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0002-0000-7000-8000-000000000130', 'cmp_active_choice_crossover', 'Hành vi Lựa chọn Quyết đoán', 'Tự tay ký hợp đồng hoặc dấn bước.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0002-0000-7000-8000-000000000131', 'cmp_thematic_mentor_intro', 'Khai mở Tuyến tình cảm', 'Gặp người phản ánh linh hồn.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0002-0000-7000-8000-000000000132', 'cmp_premise_fulfillment', 'Thực hiện Lời hứa Tiền đề', 'Những cảnh hành động/hài hước đặc trưng.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0002-0000-7000-8000-000000000133', 'cmp_false_peak_or_trough', 'Đỉnh cao / Vực sâu Ảo ảnh', 'Đảo chiều cảm xúc kịch liệt.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0002-0000-7000-8000-000000000134', 'cmp_encirclement_escalation', 'Vây hãm Leo thang', 'Áp lực dồn dập từ cả bên trong và bên ngoài.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0002-0000-7000-8000-000000000135', 'cmp_whiff_of_death', 'Mùi hương của Tử thần', 'Mất đi điểm tựa lớn nhất.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0002-0000-7000-8000-000000000136', 'cmp_mourning_and_epiphany', 'Khóc than & Ngộ đạo', 'Rũ bỏ ảo tưởng để nhìn thấy sự thật.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0002-0000-7000-8000-000000000137', 'cmp_climax_synthesis', 'Tổng hòa Đại chiến', 'Kế hoạch 5 giai đoạn lật ngược tình thế.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0002-0000-7000-8000-000000000138', 'cmp_transformed_contrast', 'Hình chiếu Chuyển hóa Đối nghịch', 'Đối chiếu trực quan với cảnh đầu.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

-- 6. Table emotional_ranges
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0002-0000-7000-8000-000000000191', 'emo_stagnant_dissatisfaction', 'Bất mãn Trì trệ')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0002-0000-7000-8000-000000000192', 'emo_abrupt_instability', 'Bất ổn Đột ngột')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0002-0000-7000-8000-000000000193', 'emo_conflicted_uncertainty', 'Giằng xé Phân vân')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0002-0000-7000-8000-000000000194', 'emo_novelty_delight', 'Thích thú Khám phá Mới')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0002-0000-7000-8000-000000000195', 'emo_illusory_confidence', 'Tự tin Hão huyền')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0002-0000-7000-8000-000000000196', 'emo_creeping_paranoia', 'Hoang mang Hoài nghi')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0002-0000-7000-8000-000000000197', 'emo_abject_despair', 'Tuyệt vọng Hoàn toàn')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0002-0000-7000-8000-000000000198', 'emo_spiritual_rebirth', 'Hồi sinh Tinh thần')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0002-0000-7000-8000-000000000199', 'emo_unshakeable_clarity', 'Sáng suốt Kiên định')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0002-0000-7000-8000-00000000019a', 'emo_transformative_closure', 'Khép lại Trọn vẹn')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

-- 7. Table event_blocks & event_block_emotionals
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0002-0000-7000-8000-0000000001f5', 'eb_stc_open', '018f0002-0000-7000-8000-00000000012d', '018f0002-0000-7000-8000-000000000065', '018f0002-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0002-0000-7000-8000-000000000262', (SELECT id FROM emotional_ranges WHERE code = 'emo_stagnant_dissatisfaction'), '018f0002-0000-7000-8000-0000000001f5')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0002-0000-7000-8000-0000000001f6', 'eb_stc_catalyst', '018f0002-0000-7000-8000-00000000012e', '018f0002-0000-7000-8000-000000000067', '018f0002-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0002-0000-7000-8000-00000000026d', (SELECT id FROM emotional_ranges WHERE code = 'emo_abrupt_instability'), '018f0002-0000-7000-8000-0000000001f6')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0002-0000-7000-8000-0000000001f7', 'eb_stc_debate', '018f0002-0000-7000-8000-00000000012f', '018f0002-0000-7000-8000-000000000068', '018f0002-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0002-0000-7000-8000-000000000278', (SELECT id FROM emotional_ranges WHERE code = 'emo_conflicted_uncertainty'), '018f0002-0000-7000-8000-0000000001f7')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0002-0000-7000-8000-0000000001f8', 'eb_stc_act2', '018f0002-0000-7000-8000-000000000130', '018f0002-0000-7000-8000-000000000069', '018f0002-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0002-0000-7000-8000-000000000283', (SELECT id FROM emotional_ranges WHERE code = 'emo_novelty_delight'), '018f0002-0000-7000-8000-0000000001f8')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0002-0000-7000-8000-0000000001f9', 'eb_stc_midpoint', '018f0002-0000-7000-8000-000000000133', '018f0002-0000-7000-8000-00000000006c', '018f0002-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0002-0000-7000-8000-00000000028e', (SELECT id FROM emotional_ranges WHERE code = 'emo_illusory_confidence'), '018f0002-0000-7000-8000-0000000001f9')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0002-0000-7000-8000-0000000001fa', 'eb_stc_badguys', '018f0002-0000-7000-8000-000000000134', '018f0002-0000-7000-8000-00000000006d', '018f0002-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0002-0000-7000-8000-000000000299', (SELECT id FROM emotional_ranges WHERE code = 'emo_creeping_paranoia'), '018f0002-0000-7000-8000-0000000001fa')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0002-0000-7000-8000-0000000001fb', 'eb_stc_all_lost', '018f0002-0000-7000-8000-000000000135', '018f0002-0000-7000-8000-00000000006e', '018f0002-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0002-0000-7000-8000-0000000002a4', (SELECT id FROM emotional_ranges WHERE code = 'emo_abject_despair'), '018f0002-0000-7000-8000-0000000001fb')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0002-0000-7000-8000-0000000001fc', 'eb_stc_dark_night', '018f0002-0000-7000-8000-000000000136', '018f0002-0000-7000-8000-00000000006f', '018f0002-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0002-0000-7000-8000-0000000002af', (SELECT id FROM emotional_ranges WHERE code = 'emo_spiritual_rebirth'), '018f0002-0000-7000-8000-0000000001fc')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0002-0000-7000-8000-0000000001fd', 'eb_stc_finale', '018f0002-0000-7000-8000-000000000137', '018f0002-0000-7000-8000-000000000071', '018f0002-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0002-0000-7000-8000-0000000002ba', (SELECT id FROM emotional_ranges WHERE code = 'emo_unshakeable_clarity'), '018f0002-0000-7000-8000-0000000001fd')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0002-0000-7000-8000-0000000001fe', 'eb_stc_final_img', '018f0002-0000-7000-8000-000000000138', '018f0002-0000-7000-8000-000000000072', '018f0002-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0002-0000-7000-8000-0000000002c5', (SELECT id FROM emotional_ranges WHERE code = 'emo_transformative_closure'), '018f0002-0000-7000-8000-0000000001fe')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

-- 8. Table event_transitions
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0002-0000-7000-8000-0000000002bd', 'Sóng gió ập đến', '018f0002-0000-7000-8000-0000000001f5', '018f0002-0000-7000-8000-0000000001f6', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0002-0000-7000-8000-0000000002be', 'Phân vân cân nhắc', '018f0002-0000-7000-8000-0000000001f6', '018f0002-0000-7000-8000-0000000001f7', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0002-0000-7000-8000-0000000002bf', 'Bước qua điểm xoay', '018f0002-0000-7000-8000-0000000001f7', '018f0002-0000-7000-8000-0000000001f8', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0002-0000-7000-8000-0000000002c0', 'Tiến lên đỉnh ảo', '018f0002-0000-7000-8000-0000000001f8', '018f0002-0000-7000-8000-0000000001f9', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0002-0000-7000-8000-0000000002c1', 'Bẫy ngầm kích hoạt', '018f0002-0000-7000-8000-0000000001f9', '018f0002-0000-7000-8000-0000000001fa', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0002-0000-7000-8000-0000000002c2', 'Đổ vỡ toàn diện', '018f0002-0000-7000-8000-0000000001fa', '018f0002-0000-7000-8000-0000000001fb', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0002-0000-7000-8000-0000000002c3', 'Chìm vào bóng tối', '018f0002-0000-7000-8000-0000000001fb', '018f0002-0000-7000-8000-0000000001fc', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0002-0000-7000-8000-0000000002c4', 'Phản công quyết liệt', '018f0002-0000-7000-8000-0000000001fc', '018f0002-0000-7000-8000-0000000001fd', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0002-0000-7000-8000-0000000002c5', 'Thiết lập trật tự mới', '018f0002-0000-7000-8000-0000000001fd', '018f0002-0000-7000-8000-0000000001fe', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

-- 9. Table constraints & motif_constraints
INSERT INTO constraints (id, code, require, reason) VALUES
  ('018f0002-0000-7000-8000-000000000321', 'stc_mirror_images', 'Opening Image và Final Image phải tương phản trực tiếp nhằm làm nổi bật sự biến đổi nhân vật.', 'Save the Cat coi sự biến đổi hình ảnh trước và sau là thước đo thành công tối thượng của cấu trúc kịch bản.')
  ON CONFLICT (code) DO UPDATE SET require = EXCLUDED.require, reason = EXCLUDED.reason;

INSERT INTO motif_constraints (id, code, constraint_id, motif_id, syntax, constrain) VALUES
  ('018f0002-0000-7000-8000-000000000385', 'save_the_cat_15_beats_stc_mirror_images', (SELECT id FROM constraints WHERE code = 'stc_mirror_images'), '018f0002-0000-7000-8000-000000000001', 'VALIDATE_MOTIF_RULE(''save_the_cat_15_beats'', ''stc_mirror_images'')', 'Opening Image và Final Image phải tương phản trực tiếp nhằm làm nổi bật sự biến đổi nhân vật.')
  ON CONFLICT (code) DO UPDATE SET syntax = EXCLUDED.syntax, constrain = EXCLUDED.constrain;

INSERT INTO constraints (id, code, require, reason) VALUES
  ('018f0002-0000-7000-8000-000000000322', 'stc_all_is_lost_whiff', 'Tại nhịp All Is Lost bắt buộc phải có Whiff of Death (mùi của tử thần).', 'Phải có cái cũ bị hủy diệt thì cái tôi mới mới có thể hồi sinh.')
  ON CONFLICT (code) DO UPDATE SET require = EXCLUDED.require, reason = EXCLUDED.reason;

INSERT INTO motif_constraints (id, code, constraint_id, motif_id, syntax, constrain) VALUES
  ('018f0002-0000-7000-8000-000000000386', 'save_the_cat_15_beats_stc_all_is_lost_whiff', (SELECT id FROM constraints WHERE code = 'stc_all_is_lost_whiff'), '018f0002-0000-7000-8000-000000000001', 'VALIDATE_MOTIF_RULE(''save_the_cat_15_beats'', ''stc_all_is_lost_whiff'')', 'Tại nhịp All Is Lost bắt buộc phải có Whiff of Death (mùi của tử thần).')
  ON CONFLICT (code) DO UPDATE SET syntax = EXCLUDED.syntax, constrain = EXCLUDED.constrain;

-- Tham chiếu nghiên cứu thị trường: + [Reedsy - Dan Harmon Story Circle: The 8-Step Storytelling Shortcut](https://reedsy.com/blog/guide/story-structure/dan-harmon-story-circle/):
-- 1. Table motifs
INSERT INTO motifs (id, code, display_name, category, version, is_system, description, min_chapter, max_chapter) VALUES
  ('018f0003-0000-7000-8000-000000000001', 'dan_harmon_story_circle', 'Vòng tròn Cốt truyện 8 Bước Dan Harmon (Harmon Story Circle)', 'plot', '1.0.0', true, 'Mô hình rút gọn cô đọng 8 bước của cấu trúc vòng lặp: You (Thế giới quen thuộc) -> Need -> Go (Ranh giới lạ) -> Search -> Find -> Take (Trả giá) -> Return -> Change.', 8, 40);

-- 2. Table motif_applicable_genres
INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f0003-0000-7000-8000-000000000015', '018f0003-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'comedy'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f0003-0000-7000-8000-000000000016', '018f0003-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'fantasy'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f0003-0000-7000-8000-000000000017', '018f0003-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'science_fiction'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

-- 3. Table state_rules
INSERT INTO state_rules (id, code, display_name) VALUES
  ('018f0003-0000-7000-8000-000000000029', 'circular_descent_ascent_rule', 'Quy tắc lặn sâu xuống vô thức và nổi lên thực tại')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO state_rules (id, code, display_name) VALUES
  ('018f0003-0000-7000-8000-00000000002a', 'cost_of_change_balance', 'Quy tắc trả giá tương xứng để nhận sự chuyển hóa')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

-- 4. Table states & state_down_by_rules
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0003-0000-7000-8000-000000000065', 'dh_you', '1. You (Nhân vật trong Vùng An toàn)', 'Khởi đầu tại nơi chốn quen thuộc, thoải mái nhưng trì trệ.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0003-0000-7000-8000-0000000000d2', '018f0003-0000-7000-8000-000000000065', (SELECT id FROM state_rules WHERE code = 'circular_descent_ascent_rule'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0003-0000-7000-8000-000000000066', 'dh_need', '2. Need (Nhu cầu / Khiếm khuyết)', 'Nhưng họ muốn điều gì đó; sự mất cân bằng thôi thúc.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0003-0000-7000-8000-0000000000dc', '018f0003-0000-7000-8000-000000000066', (SELECT id FROM state_rules WHERE code = 'circular_descent_ascent_rule'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0003-0000-7000-8000-000000000067', 'dh_go', '3. Go (Bước vào Hoàn cảnh Lạ)', 'Họ bước vào hoàn cảnh xa lạ, qua ranh giới trật tự và hỗn mang.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0003-0000-7000-8000-0000000000e6', '018f0003-0000-7000-8000-000000000067', (SELECT id FROM state_rules WHERE code = 'circular_descent_ascent_rule'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0003-0000-7000-8000-000000000068', 'dh_search', '4. Search (Thích nghi & Tìm kiếm)', 'Thích nghi với luật chơi mới, liên tục vấp ngã và thử nghiệm.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0003-0000-7000-8000-0000000000f0', '018f0003-0000-7000-8000-000000000068', (SELECT id FROM state_rules WHERE code = 'circular_descent_ascent_rule'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0003-0000-7000-8000-000000000069', 'dh_find', '5. Find (Đạt được Điều mong muốn)', 'Chạm đến cốt lõi mục tiêu tìm kiếm, nhưng là chiến thắng sớm.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0003-0000-7000-8000-0000000000fa', '018f0003-0000-7000-8000-000000000069', (SELECT id FROM state_rules WHERE code = 'circular_descent_ascent_rule'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0003-0000-7000-8000-00000000006a', 'dh_take', '6. Take (Trả một Cái giá Đắt)', 'Cái giá phải trả cho việc nắm giữ điều đó là vô cùng nặng nề.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0003-0000-7000-8000-000000000105', '018f0003-0000-7000-8000-00000000006a', (SELECT id FROM state_rules WHERE code = 'cost_of_change_balance'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0003-0000-7000-8000-00000000006b', 'dh_return', '7. Return (Quay về Nơi Xuất phát)', 'Trở lại nơi chốn quen thuộc mang theo kinh nghiệm sống sót.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0003-0000-7000-8000-00000000010e', '018f0003-0000-7000-8000-00000000006b', (SELECT id FROM state_rules WHERE code = 'circular_descent_ascent_rule'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0003-0000-7000-8000-00000000006c', 'dh_change', '8. Change (Chuyển hóa Bản thân)', 'Có khả năng thay đổi thế giới xung quanh vì bản thân đã vĩnh viễn đổi khác.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0003-0000-7000-8000-000000000119', '018f0003-0000-7000-8000-00000000006c', (SELECT id FROM state_rules WHERE code = 'cost_of_change_balance'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

-- 5. Table components
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0003-0000-7000-8000-00000000012d', 'cmp_comfort_bubble', 'Bong bóng An toàn', 'Mô tả sinh hoạt hàng ngày trước biến cố.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0003-0000-7000-8000-00000000012e', 'cmp_internal_itch', 'Cơn ngứa Ngáy Khao khát', 'Nỗi khao khát không thể kìm nén.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0003-0000-7000-8000-00000000012f', 'cmp_portal_step', 'Bước qua Cổng lạ', 'Lao vào vùng đất hoặc hoàn cảnh đối nghịch.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0003-0000-7000-8000-000000000130', 'cmp_trial_adaptation', 'Thích ứng Khắc nghiệt', 'Học cách thở dưới nước hay sinh tồn trong rừng rậm.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0003-0000-7000-8000-000000000131', 'cmp_goal_acquisition', 'Thu giữ Mục tiêu', 'Tìm thấy chiếc chìa khóa hoặc vị trí đối tượng.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0003-0000-7000-8000-000000000132', 'cmp_heavy_toll_payment', 'Thanh toán Giá đắt', 'Mất đi vũ khí, bạn đồng hành hoặc một phần danh dự.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0003-0000-7000-8000-000000000133', 'cmp_threshold_reentry', 'Tái nhập Cảnh giới', 'Quay về căn phòng quen thuộc.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0003-0000-7000-8000-000000000134', 'cmp_active_mastery', 'Làm chủ Hoàn cảnh', 'Giải quyết mâu thuẫn cũ bằng góc nhìn mới tinh.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

-- 6. Table emotional_ranges
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0003-0000-7000-8000-000000000191', 'emo_complacent_comfort', 'Thoải mái Thỏa mãn')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0003-0000-7000-8000-000000000192', 'emo_restless_craving', 'Bứt rứt Thèm khát')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0003-0000-7000-8000-000000000193', 'emo_disoriented_wonder', 'Ngỡ ngàng Choáng ngợp')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0003-0000-7000-8000-000000000194', 'emo_tenacious_struggle', 'Nỗ lực Kiên trì')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0003-0000-7000-8000-000000000195', 'emo_euphoric_discovery', 'Hưng phấn Khám phá')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0003-0000-7000-8000-000000000196', 'emo_crushing_toll', 'Đau xót Mất mát')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0003-0000-7000-8000-000000000197', 'emo_nostalgic_exhaustion', 'Kiệt sức Hoài niệm')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0003-0000-7000-8000-000000000198', 'emo_empowered_peace', 'Tự tin Thanh thản')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

-- 7. Table event_blocks & event_block_emotionals
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0003-0000-7000-8000-0000000001f5', 'eb_dh_you', '018f0003-0000-7000-8000-00000000012d', '018f0003-0000-7000-8000-000000000065', '018f0003-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0003-0000-7000-8000-000000000262', (SELECT id FROM emotional_ranges WHERE code = 'emo_complacent_comfort'), '018f0003-0000-7000-8000-0000000001f5')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0003-0000-7000-8000-0000000001f6', 'eb_dh_need', '018f0003-0000-7000-8000-00000000012e', '018f0003-0000-7000-8000-000000000066', '018f0003-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0003-0000-7000-8000-00000000026d', (SELECT id FROM emotional_ranges WHERE code = 'emo_restless_craving'), '018f0003-0000-7000-8000-0000000001f6')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0003-0000-7000-8000-0000000001f7', 'eb_dh_go', '018f0003-0000-7000-8000-00000000012f', '018f0003-0000-7000-8000-000000000067', '018f0003-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0003-0000-7000-8000-000000000278', (SELECT id FROM emotional_ranges WHERE code = 'emo_disoriented_wonder'), '018f0003-0000-7000-8000-0000000001f7')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0003-0000-7000-8000-0000000001f8', 'eb_dh_search', '018f0003-0000-7000-8000-000000000130', '018f0003-0000-7000-8000-000000000068', '018f0003-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0003-0000-7000-8000-000000000283', (SELECT id FROM emotional_ranges WHERE code = 'emo_tenacious_struggle'), '018f0003-0000-7000-8000-0000000001f8')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0003-0000-7000-8000-0000000001f9', 'eb_dh_find', '018f0003-0000-7000-8000-000000000131', '018f0003-0000-7000-8000-000000000069', '018f0003-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0003-0000-7000-8000-00000000028e', (SELECT id FROM emotional_ranges WHERE code = 'emo_euphoric_discovery'), '018f0003-0000-7000-8000-0000000001f9')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0003-0000-7000-8000-0000000001fa', 'eb_dh_take', '018f0003-0000-7000-8000-000000000132', '018f0003-0000-7000-8000-00000000006a', '018f0003-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0003-0000-7000-8000-000000000299', (SELECT id FROM emotional_ranges WHERE code = 'emo_crushing_toll'), '018f0003-0000-7000-8000-0000000001fa')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0003-0000-7000-8000-0000000001fb', 'eb_dh_return', '018f0003-0000-7000-8000-000000000133', '018f0003-0000-7000-8000-00000000006b', '018f0003-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0003-0000-7000-8000-0000000002a4', (SELECT id FROM emotional_ranges WHERE code = 'emo_nostalgic_exhaustion'), '018f0003-0000-7000-8000-0000000001fb')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0003-0000-7000-8000-0000000001fc', 'eb_dh_change', '018f0003-0000-7000-8000-000000000134', '018f0003-0000-7000-8000-00000000006c', '018f0003-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0003-0000-7000-8000-0000000002af', (SELECT id FROM emotional_ranges WHERE code = 'emo_empowered_peace'), '018f0003-0000-7000-8000-0000000001fc')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

-- 8. Table event_transitions
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0003-0000-7000-8000-0000000002bd', 'Khát khao thúc bách', '018f0003-0000-7000-8000-0000000001f5', '018f0003-0000-7000-8000-0000000001f6', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0003-0000-7000-8000-0000000002be', 'Dấn thân vùng lạ', '018f0003-0000-7000-8000-0000000001f6', '018f0003-0000-7000-8000-0000000001f7', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0003-0000-7000-8000-0000000002bf', 'Thích ứng sóng gió', '018f0003-0000-7000-8000-0000000001f7', '018f0003-0000-7000-8000-0000000001f8', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0003-0000-7000-8000-0000000002c0', 'Chạm tay vào điều ước', '018f0003-0000-7000-8000-0000000001f8', '018f0003-0000-7000-8000-0000000001f9', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0003-0000-7000-8000-0000000002c1', 'Hiến tế đẫm máu', '018f0003-0000-7000-8000-0000000001f9', '018f0003-0000-7000-8000-0000000001fa', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0003-0000-7000-8000-0000000002c2', 'Hồi hương thấm mệt', '018f0003-0000-7000-8000-0000000001fa', '018f0003-0000-7000-8000-0000000001fb', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0003-0000-7000-8000-0000000002c3', 'Bộc lộ bản lĩnh mới', '018f0003-0000-7000-8000-0000000001fb', '018f0003-0000-7000-8000-0000000001fc', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

-- 9. Table constraints & motif_constraints
INSERT INTO constraints (id, code, require, reason) VALUES
  ('018f0003-0000-7000-8000-000000000321', 'dh_loss_equilibrium', 'Tại giai đoạn Take (Bước 6), cái giá phải trả phải khiến nhân vật mất đi sự hồn nhiên hoặc một vật phẩm quý giá.', 'Không có sự mất mát thì nhân vật không có sự tái sinh ở Bước 8.')
  ON CONFLICT (code) DO UPDATE SET require = EXCLUDED.require, reason = EXCLUDED.reason;

INSERT INTO motif_constraints (id, code, constraint_id, motif_id, syntax, constrain) VALUES
  ('018f0003-0000-7000-8000-000000000385', 'dan_harmon_story_circle_dh_loss_equilibrium', (SELECT id FROM constraints WHERE code = 'dh_loss_equilibrium'), '018f0003-0000-7000-8000-000000000001', 'VALIDATE_MOTIF_RULE(''dan_harmon_story_circle'', ''dh_loss_equilibrium'')', 'Tại giai đoạn Take (Bước 6), cái giá phải trả phải khiến nhân vật mất đi sự hồn nhiên hoặc một vật phẩm quý giá.')
  ON CONFLICT (code) DO UPDATE SET syntax = EXCLUDED.syntax, constrain = EXCLUDED.constrain;

INSERT INTO constraints (id, code, require, reason) VALUES
  ('018f0003-0000-7000-8000-000000000322', 'dh_top_bottom_division', 'Nửa trên vòng tròn (1,2,7,8) là thế giới ý thức quen thuộc; nửa dưới (3,4,5,6) bắt buộc là thế giới vô thức/xa lạ.', 'Giữ đúng nguyên lý cấu trúc tự sự lưỡng phân của Dan Harmon.')
  ON CONFLICT (code) DO UPDATE SET require = EXCLUDED.require, reason = EXCLUDED.reason;

INSERT INTO motif_constraints (id, code, constraint_id, motif_id, syntax, constrain) VALUES
  ('018f0003-0000-7000-8000-000000000386', 'dan_harmon_story_circle_dh_top_bottom_division', (SELECT id FROM constraints WHERE code = 'dh_top_bottom_division'), '018f0003-0000-7000-8000-000000000001', 'VALIDATE_MOTIF_RULE(''dan_harmon_story_circle'', ''dh_top_bottom_division'')', 'Nửa trên vòng tròn (1,2,7,8) là thế giới ý thức quen thuộc; nửa dưới (3,4,5,6) bắt buộc là thế giới vô thức/xa lạ.')
  ON CONFLICT (code) DO UPDATE SET syntax = EXCLUDED.syntax, constrain = EXCLUDED.constrain;

-- Tham chiếu nghiên cứu thị trường: + [Helping Writers Become Authors - Kishōtenketsu Story Structure: What It Is, How It Works, and Examples](https://www.helpingwritersbecomeauthors.com/kishotenketsu-story-structure/):
-- 1. Table motifs
INSERT INTO motifs (id, code, display_name, category, version, is_system, description, min_chapter, max_chapter) VALUES
  ('018f0004-0000-7000-8000-000000000001', 'kishotenketsu_4_acts', 'Cấu trúc 4 Hồi Kishōtenketsu (Khởi - Thừa - Chuyển - Kết)', 'plot', '1.0.0', true, 'Cấu trúc tự sự Đông Á truyền thống không dựa trên xung đột đối kháng kiểu phương Tây mà dựa trên sự tiếp cận, đối chiếu và bất ngờ hòa quyện ý nghĩa: Ki (Khởi) -> Shō (Thừa) -> Ten (Chuyển) -> Ketsu (Kết).', 4, 50);

-- 2. Table motif_applicable_genres
INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f0004-0000-7000-8000-000000000015', '018f0004-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'slice_of_life'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f0004-0000-7000-8000-000000000016', '018f0004-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'mystery_detective'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f0004-0000-7000-8000-000000000017', '018f0004-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'literary_fiction'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

-- 3. Table state_rules
INSERT INTO state_rules (id, code, display_name) VALUES
  ('018f0004-0000-7000-8000-000000000029', 'non_conflict_juxtaposition', 'Quy tắc ghép nối phi đối kháng')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO state_rules (id, code, display_name) VALUES
  ('018f0004-0000-7000-8000-00000000002a', 'thematic_synthesis_revelation', 'Quy tắc tổng hòa ý nghĩa giác ngộ')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

-- 4. Table states & state_down_by_rules
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0004-0000-7000-8000-000000000065', 'kstk_ki', 'Ki (Khởi - Giới thiệu Hiện trạng)', 'Giới thiệu nhân vật, thời gian, không gian và nếp sống ban đầu.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0004-0000-7000-8000-0000000000d2', '018f0004-0000-7000-8000-000000000065', (SELECT id FROM state_rules WHERE code = 'non_conflict_juxtaposition'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0004-0000-7000-8000-000000000066', 'kstk_sho', 'Shō (Thừa - Mở rộng Phát triển)', 'Tiếp nối mạch câu chuyện, mở rộng chi tiết về hoàn cảnh mà không tạo xung đột lớn.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0004-0000-7000-8000-0000000000dc', '018f0004-0000-7000-8000-000000000066', (SELECT id FROM state_rules WHERE code = 'non_conflict_juxtaposition'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0004-0000-7000-8000-000000000067', 'kstk_ten', 'Ten (Chuyển - Yếu tố Bất ngờ / Đổi hướng)', 'Đột ngột xuất hiện một chủ đề, đối tượng hoặc sự kiện tưởng chừng không liên quan.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0004-0000-7000-8000-0000000000e6', '018f0004-0000-7000-8000-000000000067', (SELECT id FROM state_rules WHERE code = 'non_conflict_juxtaposition'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0004-0000-7000-8000-000000000068', 'kstk_ketsu', 'Ketsu (Kết - Hòa quyện Ý nghĩa)', 'Liên kết yếu tố Ten với Ki và Shō, tạo ra cái nhìn sâu sắc mới mẻ về cuộc đời.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0004-0000-7000-8000-0000000000f1', '018f0004-0000-7000-8000-000000000068', (SELECT id FROM state_rules WHERE code = 'thematic_synthesis_revelation'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

-- 5. Table components
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0004-0000-7000-8000-00000000012d', 'cmp_scene_introduction', 'Thiết lập Bức họa Mở màn', 'Góc nhìn tĩnh tại về cảnh vật hoặc hoạt động thường nhật.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0004-0000-7000-8000-00000000012e', 'cmp_deepening_cadence', 'Nhịp điệu Sâu sắc', 'Thêm nhân vật phụ hoặc thói quen được củng cố.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0004-0000-7000-8000-00000000012f', 'cmp_disruptive_angle', 'Góc nhìn Khác biệt Hoàn toàn', 'Một phân cảnh hoàn toàn tách biệt được lồng ghép.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0004-0000-7000-8000-000000000130', 'cmp_harmonic_unification', 'Hợp âm Hòa quyện', 'Mối dây vô hình buộc hai nửa lại với nhau.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

-- 6. Table emotional_ranges
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0004-0000-7000-8000-000000000191', 'emo_serene_curiosity', 'Hiếu kỳ Thanh thản')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0004-0000-7000-8000-000000000192', 'emo_gentle_engagement', 'Cuốn hút Dịu êm')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0004-0000-7000-8000-000000000193', 'emo_baffled_surprise', 'Ngạc nhiên Ngỡ ngàng')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0004-0000-7000-8000-000000000194', 'emo_illuminated_harmony', 'Hòa hợp Bừng sáng')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

-- 7. Table event_blocks & event_block_emotionals
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0004-0000-7000-8000-0000000001f5', 'eb_kstk_ki', '018f0004-0000-7000-8000-00000000012d', '018f0004-0000-7000-8000-000000000065', '018f0004-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0004-0000-7000-8000-000000000262', (SELECT id FROM emotional_ranges WHERE code = 'emo_serene_curiosity'), '018f0004-0000-7000-8000-0000000001f5')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0004-0000-7000-8000-0000000001f6', 'eb_kstk_sho', '018f0004-0000-7000-8000-00000000012e', '018f0004-0000-7000-8000-000000000066', '018f0004-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0004-0000-7000-8000-00000000026d', (SELECT id FROM emotional_ranges WHERE code = 'emo_gentle_engagement'), '018f0004-0000-7000-8000-0000000001f6')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0004-0000-7000-8000-0000000001f7', 'eb_kstk_ten', '018f0004-0000-7000-8000-00000000012f', '018f0004-0000-7000-8000-000000000067', '018f0004-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0004-0000-7000-8000-000000000278', (SELECT id FROM emotional_ranges WHERE code = 'emo_baffled_surprise'), '018f0004-0000-7000-8000-0000000001f7')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0004-0000-7000-8000-0000000001f8', 'eb_kstk_ketsu', '018f0004-0000-7000-8000-000000000130', '018f0004-0000-7000-8000-000000000068', '018f0004-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0004-0000-7000-8000-000000000283', (SELECT id FROM emotional_ranges WHERE code = 'emo_illuminated_harmony'), '018f0004-0000-7000-8000-0000000001f8')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

-- 8. Table event_transitions
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0004-0000-7000-8000-0000000002bd', 'Mở rộng không gian', '018f0004-0000-7000-8000-0000000001f5', '018f0004-0000-7000-8000-0000000001f6', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0004-0000-7000-8000-0000000002be', 'Góc ngoặt bất ngờ', '018f0004-0000-7000-8000-0000000001f6', '018f0004-0000-7000-8000-0000000001f7', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0004-0000-7000-8000-0000000002bf', 'Hội tụ ý nghĩa', '018f0004-0000-7000-8000-0000000001f7', '018f0004-0000-7000-8000-0000000001f8', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

-- 9. Table constraints & motif_constraints
INSERT INTO constraints (id, code, require, reason) VALUES
  ('018f0004-0000-7000-8000-000000000321', 'kstk_ten_independence', 'Hồi Ten phải mang tính bất ngờ, độc lập bề ngoài so với mạch kể của Ki và Sho trước khi được giải mã ở Ketsu.', 'Sức hấp dẫn của Kishōtenketsu đến từ sự kinh ngạc trước mối liên hệ ngầm giữa hai hình tượng.')
  ON CONFLICT (code) DO UPDATE SET require = EXCLUDED.require, reason = EXCLUDED.reason;

INSERT INTO motif_constraints (id, code, constraint_id, motif_id, syntax, constrain) VALUES
  ('018f0004-0000-7000-8000-000000000385', 'kishotenketsu_4_acts_kstk_ten_independence', (SELECT id FROM constraints WHERE code = 'kstk_ten_independence'), '018f0004-0000-7000-8000-000000000001', 'VALIDATE_MOTIF_RULE(''kishotenketsu_4_acts'', ''kstk_ten_independence'')', 'Hồi Ten phải mang tính bất ngờ, độc lập bề ngoài so với mạch kể của Ki và Sho trước khi được giải mã ở Ketsu.')
  ON CONFLICT (code) DO UPDATE SET syntax = EXCLUDED.syntax, constrain = EXCLUDED.constrain;

INSERT INTO constraints (id, code, require, reason) VALUES
  ('018f0004-0000-7000-8000-000000000322', 'kstk_no_villain_necessity', 'Cấu trúc không bắt buộc phải có nhân vật phản diện hay động lực tiêu diệt đối thủ.', 'Bảo tồn bản sắc tự sự phi xung đột triết lý Á Đông.')
  ON CONFLICT (code) DO UPDATE SET require = EXCLUDED.require, reason = EXCLUDED.reason;

INSERT INTO motif_constraints (id, code, constraint_id, motif_id, syntax, constrain) VALUES
  ('018f0004-0000-7000-8000-000000000386', 'kishotenketsu_4_acts_kstk_no_villain_necessity', (SELECT id FROM constraints WHERE code = 'kstk_no_villain_necessity'), '018f0004-0000-7000-8000-000000000001', 'VALIDATE_MOTIF_RULE(''kishotenketsu_4_acts'', ''kstk_no_villain_necessity'')', 'Cấu trúc không bắt buộc phải có nhân vật phản diện hay động lực tiêu diệt đối thủ.')
  ON CONFLICT (code) DO UPDATE SET syntax = EXCLUDED.syntax, constrain = EXCLUDED.constrain;

-- Tham chiếu nghiên cứu thị trường: + [StudioBinder - What is Freytag's Pyramid: 5 Steps of Story Structure](https://www.studiobinder.com/blog/what-is-freytags-pyramid-definition/):
-- 1. Table motifs
INSERT INTO motifs (id, code, display_name, category, version, is_system, description, min_chapter, max_chapter) VALUES
  ('018f0005-0000-7000-8000-000000000001', 'freytags_pyramid_dramatic', 'Kim tự tháp Kịch tính Freytag (Freytag''s Pyramid 5 Acts)', 'plot', '1.0.0', true, 'Mô hình bi kịch kinh điển thế kỷ 19 của Gustav Freytag gồm 5 phần đối xứng: Exposition, Rising Action, Climax, Falling Action và Catastrophe/Denouement.', 15, 75);

-- 2. Table motif_applicable_genres
INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f0005-0000-7000-8000-000000000015', '018f0005-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'literary_fiction'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f0005-0000-7000-8000-000000000017', '018f0005-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'dark'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

-- 3. Table state_rules
INSERT INTO state_rules (id, code, display_name) VALUES
  ('018f0005-0000-7000-8000-000000000029', 'hamartia_driven_escalation', 'Quy tắc leo thang do lỗi lầm định mệnh (Hamartia)')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO state_rules (id, code, display_name) VALUES
  ('018f0005-0000-7000-8000-00000000002a', 'inevitable_downward_momentum', 'Quy tắc trượt dốc tất yếu hậu đỉnh điểm')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

-- 4. Table states & state_down_by_rules
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0005-0000-7000-8000-000000000065', 'fp_exposition', '1. Exposition (Khởi đầu & Thiết lập Bối cảnh)', 'Giới thiệu nhân vật, tính cách, bối cảnh thời đại và hạt mầm xung đột ban sơ.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0005-0000-7000-8000-0000000000d2', '018f0005-0000-7000-8000-000000000065', (SELECT id FROM state_rules WHERE code = 'hamartia_driven_escalation'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0005-0000-7000-8000-000000000066', 'fp_rising_action', '2. Rising Action (Hành động Leo thang)', 'Chuỗi sự kiện gia tăng căng thẳng, tham vọng bùng cháy thúc đẩy các quyết định sai lầm.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0005-0000-7000-8000-0000000000dc', '018f0005-0000-7000-8000-000000000066', (SELECT id FROM state_rules WHERE code = 'hamartia_driven_escalation'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0005-0000-7000-8000-000000000067', 'fp_climax', '3. Climax (Đỉnh điểm Bi kịch)', 'Đỉnh chóp kim tự tháp: khoảnh khắc nhân vật phạm phải sai lầm không thể cứu vãn (Hubris/Hamartia).')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0005-0000-7000-8000-0000000000e6', '018f0005-0000-7000-8000-000000000067', (SELECT id FROM state_rules WHERE code = 'hamartia_driven_escalation'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0005-0000-7000-8000-000000000068', 'fp_falling_action', '4. Falling Action (Hành động Rơi xuống / Suy tàn)', 'Hậu quả đổ ập xuống, các thế lực liên minh tan rã, sự bất lực trước số mệnh.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0005-0000-7000-8000-0000000000f1', '018f0005-0000-7000-8000-000000000068', (SELECT id FROM state_rules WHERE code = 'inevitable_downward_momentum'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0005-0000-7000-8000-000000000069', 'fp_catastrophe', '5. Catastrophe / Dénouement (Thảm họa & Khép lại)', 'Sự sụp đổ hoàn toàn về thể xác hoặc quyền lực, tái lập trật tự đau thương nhưng công bằng.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0005-0000-7000-8000-0000000000fb', '018f0005-0000-7000-8000-000000000069', (SELECT id FROM state_rules WHERE code = 'inevitable_downward_momentum'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

-- 5. Table components
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0005-0000-7000-8000-00000000012d', 'cmp_status_and_hubris', 'Vị thế & Kiêu ngạo', 'Khắc họa uy quyền ban đầu đan xen hạt mầm mù quáng.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0005-0000-7000-8000-00000000012e', 'cmp_fatal_entanglement', 'Vướng mắc Định mệnh', 'Những toan tính mưu mô ngày càng lún sâu.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0005-0000-7000-8000-00000000012f', 'cmp_fatal_decision_apex', 'Quyết định Định mệnh Tối thượng', 'Hành vi vượt quá giới hạn đạo đức hoặc số phận.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0005-0000-7000-8000-000000000130', 'cmp_nemesis_unraveling', 'Báo ứng Giáng xuống', 'Sự phản trắc của thuộc hạ và mất dần quyền kiểm soát.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0005-0000-7000-8000-000000000131', 'cmp_tragic_catharsis', 'Thanh lọc Bi tráng', 'Cái chết của nhân vật trung tâm để lại bài học luân lý sâu sắc.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

-- 6. Table emotional_ranges
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0005-0000-7000-8000-000000000191', 'emo_sovereign_pride', 'Kiêu ngạo Quyền thế')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0005-0000-7000-8000-000000000192', 'emo_mounting_fever', 'Cơn sốt Tham vọng')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0005-0000-7000-8000-000000000193', 'emo_apocalyptic_shock', 'Chấn động Kinh hoàng')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0005-0000-7000-8000-000000000194', 'emo_inexorable_dread', 'Nỗi sợ Không thể Cưỡng lại')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0005-0000-7000-8000-000000000195', 'emo_tragic_awe', 'Kính sợ Bi tráng')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

-- 7. Table event_blocks & event_block_emotionals
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0005-0000-7000-8000-0000000001f5', 'eb_fp_expo', '018f0005-0000-7000-8000-00000000012d', '018f0005-0000-7000-8000-000000000065', '018f0005-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0005-0000-7000-8000-000000000262', (SELECT id FROM emotional_ranges WHERE code = 'emo_sovereign_pride'), '018f0005-0000-7000-8000-0000000001f5')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0005-0000-7000-8000-0000000001f6', 'eb_fp_rising', '018f0005-0000-7000-8000-00000000012e', '018f0005-0000-7000-8000-000000000066', '018f0005-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0005-0000-7000-8000-00000000026d', (SELECT id FROM emotional_ranges WHERE code = 'emo_mounting_fever'), '018f0005-0000-7000-8000-0000000001f6')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0005-0000-7000-8000-0000000001f7', 'eb_fp_climax', '018f0005-0000-7000-8000-00000000012f', '018f0005-0000-7000-8000-000000000067', '018f0005-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0005-0000-7000-8000-000000000278', (SELECT id FROM emotional_ranges WHERE code = 'emo_apocalyptic_shock'), '018f0005-0000-7000-8000-0000000001f7')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0005-0000-7000-8000-0000000001f8', 'eb_fp_falling', '018f0005-0000-7000-8000-000000000130', '018f0005-0000-7000-8000-000000000068', '018f0005-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0005-0000-7000-8000-000000000283', (SELECT id FROM emotional_ranges WHERE code = 'emo_inexorable_dread'), '018f0005-0000-7000-8000-0000000001f8')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0005-0000-7000-8000-0000000001f9', 'eb_fp_catastrophe', '018f0005-0000-7000-8000-000000000131', '018f0005-0000-7000-8000-000000000069', '018f0005-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0005-0000-7000-8000-00000000028e', (SELECT id FROM emotional_ranges WHERE code = 'emo_tragic_awe'), '018f0005-0000-7000-8000-0000000001f9')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

-- 8. Table event_transitions
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0005-0000-7000-8000-0000000002bd', 'Ngòi nổ tham vọng', '018f0005-0000-7000-8000-0000000001f5', '018f0005-0000-7000-8000-0000000001f6', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0005-0000-7000-8000-0000000002be', 'Đỉnh cao mù quáng', '018f0005-0000-7000-8000-0000000001f6', '018f0005-0000-7000-8000-0000000001f7', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0005-0000-7000-8000-0000000002bf', 'Trượt dốc không phanh', '018f0005-0000-7000-8000-0000000001f7', '018f0005-0000-7000-8000-0000000001f8', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0005-0000-7000-8000-0000000002c0', 'Bản án số phận', '018f0005-0000-7000-8000-0000000001f8', '018f0005-0000-7000-8000-0000000001f9', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

-- 9. Table constraints & motif_constraints
INSERT INTO constraints (id, code, require, reason) VALUES
  ('018f0005-0000-7000-8000-000000000321', 'fp_tragic_flaw_presence', 'Nhân vật trung tâm bắt buộc phải có một khiếm khuyết chết người (Hamartia) gây ra đỉnh điểm Climax.', 'Bản chất Kim tự tháp Freytag là kiến trúc phục vụ bi kịch nhân quả nội tại.')
  ON CONFLICT (code) DO UPDATE SET require = EXCLUDED.require, reason = EXCLUDED.reason;

INSERT INTO motif_constraints (id, code, constraint_id, motif_id, syntax, constrain) VALUES
  ('018f0005-0000-7000-8000-000000000385', 'freytags_pyramid_dramatic_fp_tragic_flaw_presence', (SELECT id FROM constraints WHERE code = 'fp_tragic_flaw_presence'), '018f0005-0000-7000-8000-000000000001', 'VALIDATE_MOTIF_RULE(''freytags_pyramid_dramatic'', ''fp_tragic_flaw_presence'')', 'Nhân vật trung tâm bắt buộc phải có một khiếm khuyết chết người (Hamartia) gây ra đỉnh điểm Climax.')
  ON CONFLICT (code) DO UPDATE SET syntax = EXCLUDED.syntax, constrain = EXCLUDED.constrain;

INSERT INTO constraints (id, code, require, reason) VALUES
  ('018f0005-0000-7000-8000-000000000322', 'fp_catharsis_rebalancing', 'Phần Catastrophe phải giải tỏa được căng thẳng đạo đức qua sự thanh tẩy bi tráng (Catharsis).', 'Giữ đúng nguyên lý thi học Aristotle và kịch nghệ Freytag.')
  ON CONFLICT (code) DO UPDATE SET require = EXCLUDED.require, reason = EXCLUDED.reason;

INSERT INTO motif_constraints (id, code, constraint_id, motif_id, syntax, constrain) VALUES
  ('018f0005-0000-7000-8000-000000000386', 'freytags_pyramid_dramatic_fp_catharsis_rebalancing', (SELECT id FROM constraints WHERE code = 'fp_catharsis_rebalancing'), '018f0005-0000-7000-8000-000000000001', 'VALIDATE_MOTIF_RULE(''freytags_pyramid_dramatic'', ''fp_catharsis_rebalancing'')', 'Phần Catastrophe phải giải tỏa được căng thẳng đạo đức qua sự thanh tẩy bi tráng (Catharsis).')
  ON CONFLICT (code) DO UPDATE SET syntax = EXCLUDED.syntax, constrain = EXCLUDED.constrain;

-- Tham chiếu nghiên cứu thị trường: + [Plottr - Using the Fichtean Curve Plot Structure for Gripping Crises](https://plottr.com/fichtean-curve-plot-structure/):
-- 1. Table motifs
INSERT INTO motifs (id, code, display_name, category, version, is_system, description, min_chapter, max_chapter) VALUES
  ('018f0006-0000-7000-8000-000000000001', 'fichtean_curve_crises', 'Đường cong Khủng hoảng Fichtean (The Fichtean Curve)', 'plot', '1.0.0', true, 'Cấu trúc nhịp độ cao liên hoàn bỏ qua phần mở đầu dài dòng, bắt đầu in media res và đẩy nhân vật qua một chuỗi các khủng hoảng leo thang không ngừng trước đại đỉnh điểm.', 18, 80);

-- 2. Table motif_applicable_genres

INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f0006-0000-7000-8000-000000000016', '018f0006-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'horror'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f0006-0000-7000-8000-000000000017', '018f0006-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'hunter_dungeon'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f0006-0000-7000-8000-000000001017', '018f0006-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'litrpg'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

-- 3. Table state_rules
INSERT INTO state_rules (id, code, display_name) VALUES
  ('018f0006-0000-7000-8000-000000000029', 'continuous_escalating_pressure', 'Quy tắc áp lực gia tăng liên tục không khoảng nghỉ lớn')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO state_rules (id, code, display_name) VALUES
  ('018f0006-0000-7000-8000-00000000002a', 'cumulative_damage_penalty', 'Quy tắc vết thương tích lũy qua từng đợt khủng hoảng')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

-- 4. Table states & state_down_by_rules
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0006-0000-7000-8000-000000000065', 'fc_in_media_res', 'Khởi đầu giữa Biến cố (In Media Res Start)', 'Nhảy thẳng vào giữa hành động; nhân vật đang trong tình huống ngặt nghèo ngay trang 1.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0006-0000-7000-8000-0000000000d2', '018f0006-0000-7000-8000-000000000065', (SELECT id FROM state_rules WHERE code = 'continuous_escalating_pressure'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0006-0000-7000-8000-000000000066', 'fc_crisis_first', 'Khủng hoảng Thứ nhất (Crisis 1 - Thoát hiểm Tạm thời)', 'Giải quyết được nguy cấp đầu tiên nhưng làm phát sinh mối nguy lớn hơn.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0006-0000-7000-8000-0000000000dc', '018f0006-0000-7000-8000-000000000066', (SELECT id FROM state_rules WHERE code = 'continuous_escalating_pressure'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0006-0000-7000-8000-000000000067', 'fc_crisis_second', 'Khủng hoảng Thứ hai (Crisis 2 - Vây bọc Thắt chặt)', 'Nguồn lực cạn kiệt, đồng minh bị cô lập, bẫy tử thần thu hẹp.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0006-0000-7000-8000-0000000000e7', '018f0006-0000-7000-8000-000000000067', (SELECT id FROM state_rules WHERE code = 'cumulative_damage_penalty'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0006-0000-7000-8000-000000000068', 'fc_crisis_third', 'Khủng hoảng Thứ ba (Crisis 3 - Bờ vực Tuyệt vọng)', 'Đỉnh điểm phụ kịch tính; mất đi phòng tuyến kiên cố cuối cùng.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0006-0000-7000-8000-0000000000f1', '018f0006-0000-7000-8000-000000000068', (SELECT id FROM state_rules WHERE code = 'cumulative_damage_penalty'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0006-0000-7000-8000-000000000069', 'fc_major_climax', 'Đại Đỉnh điểm (Main Climax)', 'Trận chiến sống còn tối hậu đối đầu trực tiếp nguồn gốc thảm họa.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0006-0000-7000-8000-0000000000fa', '018f0006-0000-7000-8000-000000000069', (SELECT id FROM state_rules WHERE code = 'continuous_escalating_pressure'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0006-0000-7000-8000-00000000006a', 'fc_rapid_falling_action', 'Thoát hiểm & Lắng đọng Nhanh (Falling Action & Resolution)', 'Nhịp thở phào nhẹ nhõm ngắn sau chuỗi sinh tử liên miên.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0006-0000-7000-8000-000000000104', '018f0006-0000-7000-8000-00000000006a', (SELECT id FROM state_rules WHERE code = 'continuous_escalating_pressure'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

-- 5. Table components
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0006-0000-7000-8000-00000000012d', 'cmp_immediate_hazard', 'Nguy cơ Trực diện Khẩn cấp', 'Chạy trốn khỏi vụ nổ hoặc sự truy đuổi sát nút.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0006-0000-7000-8000-00000000012e', 'cmp_escalated_obstacle', 'Chướng ngại Tăng cấp', 'Cửa ngõ bị khóa chặt, thời gian cạn dần.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0006-0000-7000-8000-00000000012f', 'cmp_resource_depletion', 'Cạn kiệt Tài nguyên', 'Hết đạn dược, máu cạn, thuốc men biến mất.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0006-0000-7000-8000-000000000130', 'cmp_cornered_stand', 'Dồn vào Góc tường', 'Không còn đường lui, buộc phải quay đầu tử chiến.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0006-0000-7000-8000-000000000131', 'cmp_definitive_strike', 'Đòn Đột kích Quyết tử', 'Khai thác điểm yếu chí mạng của kẻ thủ ác.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0006-0000-7000-8000-000000000132', 'cmp_exhausted_respite', 'Nghỉ ngơi Kiệt quệ', 'Ngồi giữa đống đổ nát ngắm ánh bình minh.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

-- 6. Table emotional_ranges
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0006-0000-7000-8000-000000000191', 'emo_frenetic_panic', 'Hoảng loạn Cuống cuồng')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0006-0000-7000-8000-000000000192', 'emo_breathless_tension', 'Căng thẳng Hụt hơi')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0006-0000-7000-8000-000000000193', 'emo_oppressive_exhaustion', 'Kiệt quệ Đè nặng')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0006-0000-7000-8000-000000000194', 'emo_feral_desperation', 'Tuyệt vọng Hoang dại')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0006-0000-7000-8000-000000000195', 'emo_explosive_catharsis', 'Bùng nổ Giải thoát')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0006-0000-7000-8000-000000000196', 'emo_sobering_relief', 'Nhẹ nhõm Tỉnh táo')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

-- 7. Table event_blocks & event_block_emotionals
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0006-0000-7000-8000-0000000001f5', 'eb_fc_media_res', '018f0006-0000-7000-8000-00000000012d', '018f0006-0000-7000-8000-000000000065', '018f0006-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0006-0000-7000-8000-000000000262', (SELECT id FROM emotional_ranges WHERE code = 'emo_frenetic_panic'), '018f0006-0000-7000-8000-0000000001f5')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0006-0000-7000-8000-0000000001f6', 'eb_fc_crisis1', '018f0006-0000-7000-8000-00000000012e', '018f0006-0000-7000-8000-000000000066', '018f0006-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0006-0000-7000-8000-00000000026d', (SELECT id FROM emotional_ranges WHERE code = 'emo_breathless_tension'), '018f0006-0000-7000-8000-0000000001f6')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0006-0000-7000-8000-0000000001f7', 'eb_fc_crisis2', '018f0006-0000-7000-8000-00000000012f', '018f0006-0000-7000-8000-000000000067', '018f0006-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0006-0000-7000-8000-000000000278', (SELECT id FROM emotional_ranges WHERE code = 'emo_oppressive_exhaustion'), '018f0006-0000-7000-8000-0000000001f7')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0006-0000-7000-8000-0000000001f8', 'eb_fc_crisis3', '018f0006-0000-7000-8000-000000000130', '018f0006-0000-7000-8000-000000000068', '018f0006-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0006-0000-7000-8000-000000000283', (SELECT id FROM emotional_ranges WHERE code = 'emo_feral_desperation'), '018f0006-0000-7000-8000-0000000001f8')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0006-0000-7000-8000-0000000001f9', 'eb_fc_climax', '018f0006-0000-7000-8000-000000000131', '018f0006-0000-7000-8000-000000000069', '018f0006-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0006-0000-7000-8000-00000000028e', (SELECT id FROM emotional_ranges WHERE code = 'emo_explosive_catharsis'), '018f0006-0000-7000-8000-0000000001f9')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0006-0000-7000-8000-0000000001fa', 'eb_fc_fall', '018f0006-0000-7000-8000-000000000132', '018f0006-0000-7000-8000-00000000006a', '018f0006-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0006-0000-7000-8000-000000000299', (SELECT id FROM emotional_ranges WHERE code = 'emo_sobering_relief'), '018f0006-0000-7000-8000-0000000001fa')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

-- 8. Table event_transitions
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0006-0000-7000-8000-0000000002bd', 'Thoát hiểm tạm thời', '018f0006-0000-7000-8000-0000000001f5', '018f0006-0000-7000-8000-0000000001f6', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0006-0000-7000-8000-0000000002be', 'Vòng vây thắt chặt', '018f0006-0000-7000-8000-0000000001f6', '018f0006-0000-7000-8000-0000000001f7', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0006-0000-7000-8000-0000000002bf', 'Dồn vào chân tường', '018f0006-0000-7000-8000-0000000001f7', '018f0006-0000-7000-8000-0000000001f8', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0006-0000-7000-8000-0000000002c0', 'Bùng nổ tổng lực', '018f0006-0000-7000-8000-0000000001f8', '018f0006-0000-7000-8000-0000000001f9', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0006-0000-7000-8000-0000000002c1', 'Khói bụi lắng xuống', '018f0006-0000-7000-8000-0000000001f9', '018f0006-0000-7000-8000-0000000001fa', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

-- 9. Table constraints & motif_constraints
INSERT INTO constraints (id, code, require, reason) VALUES
  ('018f0006-0000-7000-8000-000000000321', 'fc_no_prolonged_exposition', 'Không được dành quá 5% dung lượng đầu truyện cho giới thiệu bối cảnh tĩnh; phải mở màn bằng hành động trực tiếp.', 'Triết lý Fichtean Curve là giữ chặt độc giả ngay từ câu chữ đầu tiên bằng khủng hoảng tức thì.')
  ON CONFLICT (code) DO UPDATE SET require = EXCLUDED.require, reason = EXCLUDED.reason;

INSERT INTO motif_constraints (id, code, constraint_id, motif_id, syntax, constrain) VALUES
  ('018f0006-0000-7000-8000-000000000385', 'fichtean_curve_crises_fc_no_prolonged_exposition', (SELECT id FROM constraints WHERE code = 'fc_no_prolonged_exposition'), '018f0006-0000-7000-8000-000000000001', 'VALIDATE_MOTIF_RULE(''fichtean_curve_crises'', ''fc_no_prolonged_exposition'')', 'Không được dành quá 5% dung lượng đầu truyện cho giới thiệu bối cảnh tĩnh; phải mở màn bằng hành động trực tiếp.')
  ON CONFLICT (code) DO UPDATE SET syntax = EXCLUDED.syntax, constrain = EXCLUDED.constrain;

INSERT INTO constraints (id, code, require, reason) VALUES
  ('018f0006-0000-7000-8000-000000000322', 'fc_escalation_staircase', 'Mỗi đợt khủng hoảng tiếp theo bắt buộc phải có mức độ thiệt hại hoặc cọc cược cao hơn đợt trước.', 'Ngăn chặn hiện tượng tụt giảm kịch tính giữa các chương.')
  ON CONFLICT (code) DO UPDATE SET require = EXCLUDED.require, reason = EXCLUDED.reason;

INSERT INTO motif_constraints (id, code, constraint_id, motif_id, syntax, constrain) VALUES
  ('018f0006-0000-7000-8000-000000000386', 'fichtean_curve_crises_fc_escalation_staircase', (SELECT id FROM constraints WHERE code = 'fc_escalation_staircase'), '018f0006-0000-7000-8000-000000000001', 'VALIDATE_MOTIF_RULE(''fichtean_curve_crises'', ''fc_escalation_staircase'')', 'Mỗi đợt khủng hoảng tiếp theo bắt buộc phải có mức độ thiệt hại hoặc cọc cược cao hơn đợt trước.')
  ON CONFLICT (code) DO UPDATE SET syntax = EXCLUDED.syntax, constrain = EXCLUDED.constrain;

-- Tham chiếu nghiên cứu thị trường: + [MasterClass - Story Archetypes: How to Recognize Christopher Booker's 7 Basic Plots](https://www.masterclass.com/articles/a-guide-to-story-archetypes):
-- 1. Table motifs
INSERT INTO motifs (id, code, display_name, category, version, is_system, description, min_chapter, max_chapter) VALUES
  ('018f0007-0000-7000-8000-000000000001', 'booker_overcoming_the_monster', 'Hạ gục Quái thú / Thế lực Tà ác (Overcoming the Monster)', 'plot', '1.0.0', true, 'Một trong 7 cốt truyện cơ bản của Christopher Booker: Vùng đất bị đe dọa bởi một Quái vật/Ác nhân dường như bất khả chiến bại. Người anh hùng phải đối mặt trong 5 giai đoạn: Anticipation, Dream, Frustration, Nightmare và Miraculous Escape/Triumph.', 20, 90);

-- 2. Table motif_applicable_genres
INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f0007-0000-7000-8000-000000000015', '018f0007-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'fantasy'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f0007-0000-7000-8000-000000000016', '018f0007-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'horror'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f0007-0000-7000-8000-000000000017', '018f0007-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'mat_the'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f0007-0000-7000-8000-000000001017', '018f0007-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'science_fiction'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

-- 3. Table state_rules
INSERT INTO state_rules (id, code, display_name) VALUES
  ('018f0007-0000-7000-8000-000000000029', 'monster_existential_threat_rule', 'Quy tắc đe dọa sinh tồn toàn cục')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO state_rules (id, code, display_name) VALUES
  ('018f0007-0000-7000-8000-00000000002a', 'nightmare_phase_powerlessness', 'Quy tắc giai đoạn ác mộng tước đoạt toàn bộ ưu thế phàm trần')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

-- 4. Table states & state_down_by_rules
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0007-0000-7000-8000-000000000065', 'otm_anticipation', '1. Anticipation Stage & Call (Kỳ vọng & Tiếng báo nguy)', 'Ý thức về sự hiện diện của con quái vật từ xa; tiếng gọi dấn thân giải cứu cộng đồng.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0007-0000-7000-8000-0000000000d2', '018f0007-0000-7000-8000-000000000065', (SELECT id FROM state_rules WHERE code = 'monster_existential_threat_rule'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0007-0000-7000-8000-000000000066', 'otm_dream', '2. Dream Stage (Giai đoạn Mơ mộng / Thành công ban đầu)', 'Tiến quân thuận lợi, vượt qua ranh giới an toàn, tự tin đối đầu quái vật.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0007-0000-7000-8000-0000000000dc', '018f0007-0000-7000-8000-000000000066', (SELECT id FROM state_rules WHERE code = 'monster_existential_threat_rule'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0007-0000-7000-8000-000000000067', 'otm_frustration', '3. Frustration Stage (Giai đoạn Ức chế / Rơi vào Bẫy)', 'Lần đầu trực tiếp đối đầu; nhận ra sức mạnh quái thú vượt xa dự liệu, bị mắc kẹt.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0007-0000-7000-8000-0000000000e6', '018f0007-0000-7000-8000-000000000067', (SELECT id FROM state_rules WHERE code = 'monster_existential_threat_rule'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0007-0000-7000-8000-000000000068', 'otm_nightmare', '4. Nightmare Stage (Giai đoạn Ác mộng Kinh hoàng)', 'Quái thú vươn nanh vuốt áp đảo, cái chết và sự diệt vong kề cận trong gang tấc.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0007-0000-7000-8000-0000000000f1', '018f0007-0000-7000-8000-000000000068', (SELECT id FROM state_rules WHERE code = 'nightmare_phase_powerlessness'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0007-0000-7000-8000-000000000069', 'otm_miraculous_triumph', '5. The Miraculous Triumph (Kỳ tích Thắng lợi & Giải thoát)', 'Khai thác điểm mù độc nhất của quái vật hoặc bộc phát ý chí siêu phàm tiêu diệt nó.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0007-0000-7000-8000-0000000000fa', '018f0007-0000-7000-8000-000000000069', (SELECT id FROM state_rules WHERE code = 'monster_existential_threat_rule'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

-- 5. Table components
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0007-0000-7000-8000-00000000012d', 'cmp_monstrous_shadow', 'Bóng ma Quái thú', 'Tin đồn và dấu vết tàn sát rợn người khắp nơi.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0007-0000-7000-8000-00000000012e', 'cmp_confident_march', 'Cuộc hành quân Tự tin', 'Trang bị giáp trụ vũ khí tinh nhuệ sẵn sàng xuất trận.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0007-0000-7000-8000-00000000012f', 'cmp_crushing_ambush', 'Phục kích Nghiền nát', 'Vũ khí thông thường gãy vụn trước lớp giáp quái thú.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0007-0000-7000-8000-000000000130', 'cmp_desperate_survival_crawl', 'Bò trườn Tuyệt vọng', 'Đồng đội thương vong, bản thân bị dồn vào ngõ cụt.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0007-0000-7000-8000-000000000131', 'cmp_fatal_flaw_strike', 'Cú đâm Gót chân Achilles', 'Nhát chém quyết định kết liễu thực thể tà ác.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

-- 6. Table emotional_ranges
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0007-0000-7000-8000-000000000191', 'emo_righteous_zeal', 'Nhiệt huyết Chính nghĩa')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0007-0000-7000-8000-000000000192', 'emo_exhilarated_optimism', 'Lạc quan Phấn chấn')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0007-0000-7000-8000-000000000193', 'emo_chilling_dismay', 'Hoang mang Lạnh gáy')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0007-0000-7000-8000-000000000194', 'emo_paralyzing_horror', 'Kinh hoàng Tê liệt')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0007-0000-7000-8000-000000000195', 'emo_euphoric_liberation', 'Giải phóng Hân hoan')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

-- 7. Table event_blocks & event_block_emotionals
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0007-0000-7000-8000-0000000001f5', 'eb_otm_anticipate', '018f0007-0000-7000-8000-00000000012d', '018f0007-0000-7000-8000-000000000065', '018f0007-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0007-0000-7000-8000-000000000262', (SELECT id FROM emotional_ranges WHERE code = 'emo_righteous_zeal'), '018f0007-0000-7000-8000-0000000001f5')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0007-0000-7000-8000-0000000001f6', 'eb_otm_dream', '018f0007-0000-7000-8000-00000000012e', '018f0007-0000-7000-8000-000000000066', '018f0007-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0007-0000-7000-8000-00000000026d', (SELECT id FROM emotional_ranges WHERE code = 'emo_exhilarated_optimism'), '018f0007-0000-7000-8000-0000000001f6')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0007-0000-7000-8000-0000000001f7', 'eb_otm_frustrate', '018f0007-0000-7000-8000-00000000012f', '018f0007-0000-7000-8000-000000000067', '018f0007-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0007-0000-7000-8000-000000000278', (SELECT id FROM emotional_ranges WHERE code = 'emo_chilling_dismay'), '018f0007-0000-7000-8000-0000000001f7')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0007-0000-7000-8000-0000000001f8', 'eb_otm_nightmare', '018f0007-0000-7000-8000-000000000130', '018f0007-0000-7000-8000-000000000068', '018f0007-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0007-0000-7000-8000-000000000283', (SELECT id FROM emotional_ranges WHERE code = 'emo_paralyzing_horror'), '018f0007-0000-7000-8000-0000000001f8')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0007-0000-7000-8000-0000000001f9', 'eb_otm_triumph', '018f0007-0000-7000-8000-000000000131', '018f0007-0000-7000-8000-000000000069', '018f0007-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0007-0000-7000-8000-00000000028e', (SELECT id FROM emotional_ranges WHERE code = 'emo_euphoric_liberation'), '018f0007-0000-7000-8000-0000000001f9')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

-- 8. Table event_transitions
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0007-0000-7000-8000-0000000002bd', 'Tiến quân xuất trận', '018f0007-0000-7000-8000-0000000001f5', '018f0007-0000-7000-8000-0000000001f6', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0007-0000-7000-8000-0000000002be', 'Đụng độ bức tường thép', '018f0007-0000-7000-8000-0000000001f6', '018f0007-0000-7000-8000-0000000001f7', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0007-0000-7000-8000-0000000002bf', 'Rơi vào miệng quái vật', '018f0007-0000-7000-8000-0000000001f7', '018f0007-0000-7000-8000-0000000001f8', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0007-0000-7000-8000-0000000002c0', 'Đòn kỳ tích lật ngược thế cờ', '018f0007-0000-7000-8000-0000000001f8', '018f0007-0000-7000-8000-0000000001f9', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

-- 9. Table constraints & motif_constraints
INSERT INTO constraints (id, code, require, reason) VALUES
  ('018f0007-0000-7000-8000-000000000321', 'otm_monster_asymmetry', 'Quái vật/Ác nhân phải sở hữu sức mạnh áp đảo vượt trội gấp nhiều lần nhân vật chính ở giai đoạn đầu và giữa.', 'Tạo ra sức nặng đạo đức và cảm giác kỳ tích chân thật khi đánh bại quái vật.')
  ON CONFLICT (code) DO UPDATE SET require = EXCLUDED.require, reason = EXCLUDED.reason;

INSERT INTO motif_constraints (id, code, constraint_id, motif_id, syntax, constrain) VALUES
  ('018f0007-0000-7000-8000-000000000385', 'booker_overcoming_the_monster_otm_monster_asymmetry', (SELECT id FROM constraints WHERE code = 'otm_monster_asymmetry'), '018f0007-0000-7000-8000-000000000001', 'VALIDATE_MOTIF_RULE(''booker_overcoming_the_monster'', ''otm_monster_asymmetry'')', 'Quái vật/Ác nhân phải sở hữu sức mạnh áp đảo vượt trội gấp nhiều lần nhân vật chính ở giai đoạn đầu và giữa.')
  ON CONFLICT (code) DO UPDATE SET syntax = EXCLUDED.syntax, constrain = EXCLUDED.constrain;

INSERT INTO constraints (id, code, require, reason) VALUES
  ('018f0007-0000-7000-8000-000000000322', 'otm_communal_stakes', 'Sự thất bại của nhân vật chính phải đồng nghĩa với sự hủy diệt của toàn bộ cộng đồng hoặc vương quốc.', 'Giữ đúng tầm vóc sử thi của cổ mẫu Christopher Booker.')
  ON CONFLICT (code) DO UPDATE SET require = EXCLUDED.require, reason = EXCLUDED.reason;

INSERT INTO motif_constraints (id, code, constraint_id, motif_id, syntax, constrain) VALUES
  ('018f0007-0000-7000-8000-000000000386', 'booker_overcoming_the_monster_otm_communal_stakes', (SELECT id FROM constraints WHERE code = 'otm_communal_stakes'), '018f0007-0000-7000-8000-000000000001', 'VALIDATE_MOTIF_RULE(''booker_overcoming_the_monster'', ''otm_communal_stakes'')', 'Sự thất bại của nhân vật chính phải đồng nghĩa với sự hủy diệt của toàn bộ cộng đồng hoặc vương quốc.')
  ON CONFLICT (code) DO UPDATE SET syntax = EXCLUDED.syntax, constrain = EXCLUDED.constrain;

-- Tham chiếu nghiên cứu thị trường: + [Story Grid - The Five Commandments of Storytelling](https://storygrid.com/articles/five-commandments-of-storytelling):
-- 1. Table motifs
INSERT INTO motifs (id, code, display_name, category, version, is_system, description, min_chapter, max_chapter) VALUES
  ('018f0008-0000-7000-8000-000000000001', 'story_grid_five_commandments', 'Năm Điều răn Tự sự của Story Grid (Five Commandments of Storytelling)', 'plot', '1.0.0', true, 'Khung phương pháp luận biên tập chuyên sâu của Shawn Coyne: Mỗi cảnh (Scene), trường đoạn (Sequence) hay toàn bộ tác phẩm đều phải tuân thủ 5 điều răn: Inciting Incident, Turning Point Progressive Complication, Crisis, Climax và Resolution.', 10, 70);

-- 2. Table motif_applicable_genres
INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f0008-0000-7000-8000-000000000015', '018f0008-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'mystery_detective'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

-- 3. Table state_rules
INSERT INTO state_rules (id, code, display_name) VALUES
  ('018f0008-0000-7000-8000-000000000029', 'value_shift_requirement', 'Quy tắc bắt buộc chuyển dịch giá trị từ đầu đến cuối cảnh')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO state_rules (id, code, display_name) VALUES
  ('018f0008-0000-7000-8000-00000000002a', 'irreversible_best_bad_choice', 'Quy tắc khủng hoảng buộc chọn điều ít tồi tệ hơn')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

-- 4. Table states & state_down_by_rules
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0008-0000-7000-8000-000000000065', 'sg_inciting_incident', '1. Sự cố Khởi xướng (Inciting Incident)', 'Phá vỡ trạng thái cân bằng của nhân vật, hoặc do sự việc ngẫu nhiên (Causal) hoặc do hành vi chủ động.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0008-0000-7000-8000-0000000000d2', '018f0008-0000-7000-8000-000000000065', (SELECT id FROM state_rules WHERE code = 'value_shift_requirement'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0008-0000-7000-8000-000000000066', 'sg_turning_point', '2. Điểm xoay Biến chứng Tiến triển (Turning Point Complication)', 'Một hành động của nhân vật khác hoặc một tiết lộ bất ngờ (Action/Revelation) làm tắc nghẽn kế hoạch cũ.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0008-0000-7000-8000-0000000000dc', '018f0008-0000-7000-8000-000000000066', (SELECT id FROM state_rules WHERE code = 'value_shift_requirement'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0008-0000-7000-8000-000000000067', 'sg_crisis_question', '3. Câu hỏi Khủng hoảng (The Crisis Question)', 'Một tình thế tiến thoái lưỡng nan: Chọn điều tốt hơn giữa hai điều tốt, hoặc chọn điều ít tệ hơn (Best Bad Choice).')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0008-0000-7000-8000-0000000000e7', '018f0008-0000-7000-8000-000000000067', (SELECT id FROM state_rules WHERE code = 'irreversible_best_bad_choice'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0008-0000-7000-8000-000000000068', 'sg_climax_action', '4. Hành động Đỉnh điểm (The Climax Choice)', 'Nhân vật dứt khoát đưa ra quyết định hành động dưới áp lực lớn nhất.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0008-0000-7000-8000-0000000000f0', '018f0008-0000-7000-8000-000000000068', (SELECT id FROM state_rules WHERE code = 'value_shift_requirement'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0008-0000-7000-8000-000000000069', 'sg_resolution_new_order', '5. Giải quyết & Thiết lập Trật tự Mới (Resolution)', 'Hậu quả của quyết định được bộc lộ, tạo ra một giá trị mới cho cảnh hoặc toàn truyện.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0008-0000-7000-8000-0000000000fa', '018f0008-0000-7000-8000-000000000069', (SELECT id FROM state_rules WHERE code = 'value_shift_requirement'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

-- 5. Table components
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0008-0000-7000-8000-00000000012d', 'cmp_equilibrium_disruptor', 'Tác nhân Phá vỡ Cân bằng', 'Vụ án mạng xảy ra hoặc nhận được cuộc gọi tống tiền.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0008-0000-7000-8000-00000000012e', 'cmp_unexpected_revelation', 'Tiết lộ Trái khoáy Bất ngờ', 'Manh mối chính biến thành bằng chứng ngoại phạm của nghi phạm.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0008-0000-7000-8000-00000000012f', 'cmp_dilemma_crucible', 'Lò nung Tiến thoái Lưỡng nan', 'Phải chọn cứu con tin hay giữ bảo mật quốc gia.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0008-0000-7000-8000-000000000130', 'cmp_committed_verdict', 'Hành vi Đột phá Dứt khoát', 'Lập tức nổ súng hoặc công bố tài liệu mật.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0008-0000-7000-8000-000000000131', 'cmp_stabilized_aftermath', 'Hiện trạng Cân bằng Mới', 'Sự thật được định hình, kẻ phạm tội bị còng tay.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

-- 6. Table emotional_ranges
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0008-0000-7000-8000-000000000191', 'emo_alert_intrigued', 'Cảnh giác Tò mò')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0008-0000-7000-8000-000000000192', 'emo_destabilized_shock', 'Chấn động Mất phương hướng')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0008-0000-7000-8000-000000000193', 'emo_agonizing_dilemma', 'Giằng xé Khắc khoải')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0008-0000-7000-8000-000000000194', 'emo_resolute_execution', 'Kiên quyết Thực thi')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0008-0000-7000-8000-000000000195', 'emo_calibrated_settlement', 'Định hình Lắng đọng')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

-- 7. Table event_blocks & event_block_emotionals
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0008-0000-7000-8000-0000000001f5', 'eb_sg_incite', '018f0008-0000-7000-8000-00000000012d', '018f0008-0000-7000-8000-000000000065', '018f0008-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0008-0000-7000-8000-000000000262', (SELECT id FROM emotional_ranges WHERE code = 'emo_alert_intrigued'), '018f0008-0000-7000-8000-0000000001f5')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0008-0000-7000-8000-0000000001f6', 'eb_sg_turn', '018f0008-0000-7000-8000-00000000012e', '018f0008-0000-7000-8000-000000000066', '018f0008-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0008-0000-7000-8000-00000000026d', (SELECT id FROM emotional_ranges WHERE code = 'emo_destabilized_shock'), '018f0008-0000-7000-8000-0000000001f6')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0008-0000-7000-8000-0000000001f7', 'eb_sg_crisis', '018f0008-0000-7000-8000-00000000012f', '018f0008-0000-7000-8000-000000000067', '018f0008-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0008-0000-7000-8000-000000000278', (SELECT id FROM emotional_ranges WHERE code = 'emo_agonizing_dilemma'), '018f0008-0000-7000-8000-0000000001f7')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0008-0000-7000-8000-0000000001f8', 'eb_sg_climax', '018f0008-0000-7000-8000-000000000130', '018f0008-0000-7000-8000-000000000068', '018f0008-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0008-0000-7000-8000-000000000283', (SELECT id FROM emotional_ranges WHERE code = 'emo_resolute_execution'), '018f0008-0000-7000-8000-0000000001f8')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0008-0000-7000-8000-0000000001f9', 'eb_sg_resolve', '018f0008-0000-7000-8000-000000000131', '018f0008-0000-7000-8000-000000000069', '018f0008-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0008-0000-7000-8000-00000000028e', (SELECT id FROM emotional_ranges WHERE code = 'emo_calibrated_settlement'), '018f0008-0000-7000-8000-0000000001f9')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

-- 8. Table event_transitions
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0008-0000-7000-8000-0000000002bd', 'Xung đột bùng nổ phức tạp', '018f0008-0000-7000-8000-0000000001f5', '018f0008-0000-7000-8000-0000000001f6', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0008-0000-7000-8000-0000000002be', 'Đẩy vào bước đường cùng', '018f0008-0000-7000-8000-0000000001f6', '018f0008-0000-7000-8000-0000000001f7', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0008-0000-7000-8000-0000000002bf', 'Quyết định sinh tử', '018f0008-0000-7000-8000-0000000001f7', '018f0008-0000-7000-8000-0000000001f8', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0008-0000-7000-8000-0000000002c0', 'Trật tự mới hình thành', '018f0008-0000-7000-8000-0000000001f8', '018f0008-0000-7000-8000-0000000001f9', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

-- 9. Table constraints & motif_constraints
INSERT INTO constraints (id, code, require, reason) VALUES
  ('018f0008-0000-7000-8000-000000000321', 'sg_crisis_binary_nature', 'Khủng hoảng Crisis bắt buộc phải là một câu hỏi nhị phân rõ ràng đòi hỏi hành động lựa chọn cụ thể.', 'Ngăn ngừa việc câu chuyện trôi dạt vô định thiếu động lực nhân vật.')
  ON CONFLICT (code) DO UPDATE SET require = EXCLUDED.require, reason = EXCLUDED.reason;

INSERT INTO motif_constraints (id, code, constraint_id, motif_id, syntax, constrain) VALUES
  ('018f0008-0000-7000-8000-000000000385', 'story_grid_five_commandments_sg_crisis_binary_nature', (SELECT id FROM constraints WHERE code = 'sg_crisis_binary_nature'), '018f0008-0000-7000-8000-000000000001', 'VALIDATE_MOTIF_RULE(''story_grid_five_commandments'', ''sg_crisis_binary_nature'')', 'Khủng hoảng Crisis bắt buộc phải là một câu hỏi nhị phân rõ ràng đòi hỏi hành động lựa chọn cụ thể.')
  ON CONFLICT (code) DO UPDATE SET syntax = EXCLUDED.syntax, constrain = EXCLUDED.constrain;

INSERT INTO constraints (id, code, require, reason) VALUES
  ('018f0008-0000-7000-8000-000000000322', 'sg_value_shift_nonzero', 'Trạng thái Resolution bắt buộc phải có giá trị sống/chết, danh dự/nhục nhã bị dịch chuyển so với Inciting Incident.', 'Đảm bảo mỗi đơn vị kịch bản đều tạo ra tiến trình thực chất cho toàn mạch tự sự.')
  ON CONFLICT (code) DO UPDATE SET require = EXCLUDED.require, reason = EXCLUDED.reason;

INSERT INTO motif_constraints (id, code, constraint_id, motif_id, syntax, constrain) VALUES
  ('018f0008-0000-7000-8000-000000000386', 'story_grid_five_commandments_sg_value_shift_nonzero', (SELECT id FROM constraints WHERE code = 'sg_value_shift_nonzero'), '018f0008-0000-7000-8000-000000000001', 'VALIDATE_MOTIF_RULE(''story_grid_five_commandments'', ''sg_value_shift_nonzero'')', 'Trạng thái Resolution bắt buộc phải có giá trị sống/chết, danh dự/nhục nhã bị dịch chuyển so với Inciting Incident.')
  ON CONFLICT (code) DO UPDATE SET syntax = EXCLUDED.syntax, constrain = EXCLUDED.constrain;

-- Tham chiếu nghiên cứu thị trường: + [TV Tropes - Spirit Cultivation Genre: The Trash Underdog Trope](https://tvtropes.org/pmwiki/pmwiki.php/Main/SpiritCultivationGenre):
-- 1. Table motifs
INSERT INTO motifs (id, code, display_name, category, version, is_system, description, min_chapter, max_chapter) VALUES
  ('018f0009-0000-7000-8000-000000000001', 'phe_vat_nghich_tap_xianxia', 'Phế vật Nghịch tập (Underdog to Sovereign Trope)', 'plot', '1.0.0', true, 'Mô-típ cốt lõi của dòng Tiên hiệp/Huyền huyễn mạng Trung Quốc: Nhân vật chính ban đầu đan điền tổn hại, linh căn rác rưởi bị tông tộc khinh khi; nhờ kỳ ngộ bảo vật nghịch thiên, từng bước âm thầm tu luyện, vả mặt thiên kiêu và xưng bá vạn giới.', 50, 500);

-- 2. Table motif_applicable_genres
INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f0009-0000-7000-8000-000000000015', '018f0009-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'xianxia'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f0009-0000-7000-8000-000000000016', '018f0009-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'xuanhuan'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f0009-0000-7000-8000-000000000017', '018f0009-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'progression'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

-- 3. Table state_rules
INSERT INTO state_rules (id, code, display_name) VALUES
  ('018f0009-0000-7000-8000-000000000029', 'face_slapping_retaliation_rule', 'Quy tắc vả mặt tương xứng với nhục nhã ban đầu')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO state_rules (id, code, display_name) VALUES
  ('018f0009-0000-7000-8000-00000000002a', 'realm_breakthrough_threshold', 'Quy tắc bình cảnh và vượt cấp khiêu chiến')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

-- 4. Table states & state_down_by_rules
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0009-0000-7000-8000-000000000065', 'pv_humiliated_cripple', '1. Phế căn & Ô nhục (Humiliated Trash Phase)', 'Đan điền bị phế, hôn thê hủy hôn, tộc nhân sỉ nhục, phụ thân chịu áp lực.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0009-0000-7000-8000-0000000000d2', '018f0009-0000-7000-8000-000000000065', (SELECT id FROM state_rules WHERE code = 'face_slapping_retaliation_rule'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0009-0000-7000-8000-000000000066', 'pv_fortuitous_relic_awakening', '2. Cơ duyên Nghịch thiên (Relic & Master Awakening)', 'Nhặt được nhẫn cổ chứa tàn hồn cường giả hoặc Thần thạch thượng cổ tái tạo kinh mạch.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0009-0000-7000-8000-0000000000dc', '018f0009-0000-7000-8000-000000000066', (SELECT id FROM state_rules WHERE code = 'face_slapping_retaliation_rule'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0009-0000-7000-8000-000000000067', 'pv_clandestine_grinding', '3. Ẩn nhẫn Khổ tu (Clandestine Grinding & Alchemy)', 'Ẩn giấu thực lực, học thuật luyện đan, rèn đúc thần thể vượt xa cực hạn thế tục.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0009-0000-7000-8000-0000000000e7', '018f0009-0000-7000-8000-000000000067', (SELECT id FROM state_rules WHERE code = 'realm_breakthrough_threshold'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0009-0000-7000-8000-000000000068', 'pv_clan_tournament_face_slap', '4. Đại bỉ Gia tộc Vả mặt (Clan Tournament Face-Slap)', 'Tung hoành trên lôi đài, một chiêu trấn áp kẻ từng khinh khi mình, đoạt vị trí quán quân.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0009-0000-7000-8000-0000000000f0', '018f0009-0000-7000-8000-000000000068', (SELECT id FROM state_rules WHERE code = 'face_slapping_retaliation_rule'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0009-0000-7000-8000-000000000069', 'pv_sect_genius_crushing', '5. Trấn áp Thiên kiêu Tông môn (Sect Sovereign Ascension)', 'Bước ra thế giới lớn, lấy yếu thắng mạnh vượt cấp khiêu chiến, mở ra con đường vô địch.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0009-0000-7000-8000-0000000000fb', '018f0009-0000-7000-8000-000000000069', (SELECT id FROM state_rules WHERE code = 'realm_breakthrough_threshold'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

-- 5. Table components
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0009-0000-7000-8000-00000000012d', 'cmp_engagement_annulment', 'Thư Hủy hôn & Lời thề 3 Năm', 'Thiếu nữ ngạo mạn tới cửa hủy ước gây nên vết nhơ danh dự gia tộc.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0009-0000-7000-8000-00000000012e', 'cmp_ancient_ring_resonance', 'Cổ giới Linh quang Thức tỉnh', 'Giọt máu nhỏ vào di vật đánh thức lão gia gia thông tuệ vạn cổ.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0009-0000-7000-8000-00000000012f', 'cmp_bone_marrow_cleansing', 'Tẩy tủy Phạt mao Tái tạo', 'Chịu đựng nỗi đau xé thịt nát xương để rèn nên Hỗn Độn Thể.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0009-0000-7000-8000-000000000130', 'cmp_shockwave_arena_strike', 'Kình phong Chấn động Lôi đài', 'Tung chiêu thức nghịch thiên khiến toàn trường há hốc kinh ngạc.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0009-0000-7000-8000-000000000131', 'cmp_supreme_domain_unleash', 'Khai mở Lĩnh vực Chí tôn', 'Nghiền nát thần tử các đại giáo phái, đạp thiên kiêu dưới chân.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

-- 6. Table emotional_ranges
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0009-0000-7000-8000-000000000191', 'emo_bitter_humiliation', 'Nhục nhã Uất hận')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0009-0000-7000-8000-000000000192', 'emo_fierce_vindication_seed', 'Hạt mầm Phấn chấn Nghịch thiên')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0009-0000-7000-8000-000000000193', 'emo_iron_will_solitude', 'Cô độc Sắt đá')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0009-0000-7000-8000-000000000194', 'emo_ecstatic_face_slap_catharsis', 'Khoái cảm Vả mặt Sảng khoái')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0009-0000-7000-8000-000000000195', 'emo_arrogant_domination', 'Ngạo nghễ Độc tôn')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

-- 7. Table event_blocks & event_block_emotionals
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0009-0000-7000-8000-0000000001f5', 'eb_pv_trash', '018f0009-0000-7000-8000-00000000012d', '018f0009-0000-7000-8000-000000000065', '018f0009-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0009-0000-7000-8000-000000000262', (SELECT id FROM emotional_ranges WHERE code = 'emo_bitter_humiliation'), '018f0009-0000-7000-8000-0000000001f5')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0009-0000-7000-8000-0000000001f6', 'eb_pv_relic', '018f0009-0000-7000-8000-00000000012e', '018f0009-0000-7000-8000-000000000066', '018f0009-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0009-0000-7000-8000-00000000026d', (SELECT id FROM emotional_ranges WHERE code = 'emo_fierce_vindication_seed'), '018f0009-0000-7000-8000-0000000001f6')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0009-0000-7000-8000-0000000001f7', 'eb_pv_grind', '018f0009-0000-7000-8000-00000000012f', '018f0009-0000-7000-8000-000000000067', '018f0009-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0009-0000-7000-8000-000000000278', (SELECT id FROM emotional_ranges WHERE code = 'emo_iron_will_solitude'), '018f0009-0000-7000-8000-0000000001f7')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0009-0000-7000-8000-0000000001f8', 'eb_pv_tourney', '018f0009-0000-7000-8000-000000000130', '018f0009-0000-7000-8000-000000000068', '018f0009-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0009-0000-7000-8000-000000000283', (SELECT id FROM emotional_ranges WHERE code = 'emo_ecstatic_face_slap_catharsis'), '018f0009-0000-7000-8000-0000000001f8')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0009-0000-7000-8000-0000000001f9', 'eb_pv_ascend', '018f0009-0000-7000-8000-000000000131', '018f0009-0000-7000-8000-000000000069', '018f0009-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0009-0000-7000-8000-00000000028e', (SELECT id FROM emotional_ranges WHERE code = 'emo_arrogant_domination'), '018f0009-0000-7000-8000-0000000001f9')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

-- 8. Table event_transitions
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0009-0000-7000-8000-0000000002bd', 'Kỳ ngộ thức tỉnh giữa tuyệt lộ', '018f0009-0000-7000-8000-0000000001f5', '018f0009-0000-7000-8000-0000000001f6', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0009-0000-7000-8000-0000000002be', 'Nuốt đắng bế quan tu luyện', '018f0009-0000-7000-8000-0000000001f6', '018f0009-0000-7000-8000-0000000001f7', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0009-0000-7000-8000-0000000002bf', 'Xuất quan trấn áp lôi đài', '018f0009-0000-7000-8000-0000000001f7', '018f0009-0000-7000-8000-0000000001f8', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0009-0000-7000-8000-0000000002c0', 'Khiêu chiến thiên kiêu đại vực', '018f0009-0000-7000-8000-0000000001f8', '018f0009-0000-7000-8000-0000000001f9', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

-- 9. Table constraints & motif_constraints
INSERT INTO constraints (id, code, require, reason) VALUES
  ('018f0009-0000-7000-8000-000000000321', 'pv_must_suffer_unjust_humiliation', 'Giai đoạn đầu bắt buộc phải có ít nhất một hành vi coi thường danh dự bất công (hủy hôn, đuổi khỏi gia tộc).', 'Tạo ra điện thế cảm xúc phẫn nộ làm đòn bẩy cho khoái cảm sảng văn ở giai đoạn sau.')
  ON CONFLICT (code) DO UPDATE SET require = EXCLUDED.require, reason = EXCLUDED.reason;

INSERT INTO motif_constraints (id, code, constraint_id, motif_id, syntax, constrain) VALUES
  ('018f0009-0000-7000-8000-000000000385', 'phe_vat_nghich_tap_xianxia_pv_must_suffer_unjust_humiliation', (SELECT id FROM constraints WHERE code = 'pv_must_suffer_unjust_humiliation'), '018f0009-0000-7000-8000-000000000001', 'VALIDATE_MOTIF_RULE(''phe_vat_nghich_tap_xianxia'', ''pv_must_suffer_unjust_humiliation'')', 'Giai đoạn đầu bắt buộc phải có ít nhất một hành vi coi thường danh dự bất công (hủy hôn, đuổi khỏi gia tộc).')
  ON CONFLICT (code) DO UPDATE SET syntax = EXCLUDED.syntax, constrain = EXCLUDED.constrain;

INSERT INTO constraints (id, code, require, reason) VALUES
  ('018f0009-0000-7000-8000-000000000322', 'pv_relic_exclusivity', 'Cơ duyên/Bảo vật ban đầu phải mang tính duy nhất và chỉ nhân vật chính mới kích hoạt được.', 'Khẳng định thiên mệnh và sự khác biệt tuyệt đối của nhân vật chính.')
  ON CONFLICT (code) DO UPDATE SET require = EXCLUDED.require, reason = EXCLUDED.reason;

INSERT INTO motif_constraints (id, code, constraint_id, motif_id, syntax, constrain) VALUES
  ('018f0009-0000-7000-8000-000000000386', 'phe_vat_nghich_tap_xianxia_pv_relic_exclusivity', (SELECT id FROM constraints WHERE code = 'pv_relic_exclusivity'), '018f0009-0000-7000-8000-000000000001', 'VALIDATE_MOTIF_RULE(''phe_vat_nghich_tap_xianxia'', ''pv_relic_exclusivity'')', 'Cơ duyên/Bảo vật ban đầu phải mang tính duy nhất và chỉ nhân vật chính mới kích hoạt được.')
  ON CONFLICT (code) DO UPDATE SET syntax = EXCLUDED.syntax, constrain = EXCLUDED.constrain;

-- Tham chiếu nghiên cứu thị trường: + [TV Tropes - LitRPG: The Game System Progression Trope](https://tvtropes.org/pmwiki/pmwiki.php/Main/LitRPG):
-- 1. Table motifs
INSERT INTO motifs (id, code, display_name, category, version, is_system, description, min_chapter, max_chapter) VALUES
  ('018f000a-0000-7000-8000-000000000001', 'he_thong_luu_system_progression', 'Hệ thống lưu (System LitRPG Progression)', 'plot', '1.0.0', true, 'Mô-típ số hóa sức mạnh đặc trưng: Nhân vật chính thức tỉnh Giao diện Hệ thống độc quyền (Status Screen, Quest Log, Gacha, Skill Points); hoàn thành nhiệm vụ để nhận phần thưởng buff vượt cấp và thao túng quy tắc thế giới.', 30, 400);

-- 2. Table motif_applicable_genres
INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f000a-0000-7000-8000-000000000015', '018f000a-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'litrpg'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f000a-0000-7000-8000-000000000016', '018f000a-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'di_nang'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f000a-0000-7000-8000-000000000017', '018f000a-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'xuyen_khong'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f000a-0000-7000-8000-000000001017', '018f000a-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'di_gioi'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

-- 3. Table state_rules
INSERT INTO state_rules (id, code, display_name) VALUES
  ('018f000a-0000-7000-8000-000000000029', 'numerical_stat_consistency', 'Quy tắc tính nhất quán của bảng chỉ số số học')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO state_rules (id, code, display_name) VALUES
  ('018f000a-0000-7000-8000-00000000002a', 'quest_penalty_urgency_rule', 'Quy tắc áp lực từ hình phạt nhiệm vụ thất bại')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

-- 4. Table states & state_down_by_rules
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f000a-0000-7000-8000-000000000065', 'sys_binding_awakening', '1. Thức tỉnh Ràng buộc Hệ thống (System Binding)', 'Tiếng ''Ding!'' báo hiệu kích hoạt bảng chỉ số giao diện nổi trước mắt giữa cơn nguy kịch.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f000a-0000-7000-8000-0000000000d2', '018f000a-0000-7000-8000-000000000065', (SELECT id FROM state_rules WHERE code = 'numerical_stat_consistency'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f000a-0000-7000-8000-000000000066', 'sys_novice_quest_grinding', '2. Cày cuốc Nhiệm vụ Tân thủ (Novice Grinding)', 'Nhận nhiệm vụ hàng ngày: chống đẩy, tiêu diệt quái nhỏ, tích lũy điểm EXP và vàng ban đầu.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f000a-0000-7000-8000-0000000000dc', '018f000a-0000-7000-8000-000000000066', (SELECT id FROM state_rules WHERE code = 'numerical_stat_consistency'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f000a-0000-7000-8000-000000000067', 'sys_penalty_quest_crisis', '3. Khủng hoảng Nhiệm vụ Trừng phạt (Penalty Quest Threat)', 'Hệ thống đưa ra nhiệm vụ cưỡng chế với hình phạt trí mạng nếu không hoàn thành đúng hạn.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f000a-0000-7000-8000-0000000000e7', '018f000a-0000-7000-8000-000000000067', (SELECT id FROM state_rules WHERE code = 'quest_penalty_urgency_rule'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f000a-0000-7000-8000-000000000068', 'sys_gacha_breakthrough', '4. Quay thưởng Kỳ tích / Đột phá Kỹ năng (S-Rank Gacha Awakening)', 'Rút trúng kỹ năng cấp Thần độc nhất vô nhị (SSS-Rank Skill) đảo lộn toàn bộ logic trò chơi.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f000a-0000-7000-8000-0000000000f0', '018f000a-0000-7000-8000-000000000068', (SELECT id FROM state_rules WHERE code = 'numerical_stat_consistency'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f000a-0000-7000-8000-000000000069', 'sys_rule_transcendence', '5. Vượt thoát Quyền hạn Hệ thống (System Authority Transcendence)', 'Phát hiện bí mật đằng sau nguồn gốc của hệ thống và trở thành kẻ kiểm soát quy tắc tối cao.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f000a-0000-7000-8000-0000000000fa', '018f000a-0000-7000-8000-000000000069', (SELECT id FROM state_rules WHERE code = 'numerical_stat_consistency'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

-- 5. Table components
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f000a-0000-7000-8000-00000000012d', 'cmp_status_screen_pop', 'Bảng Thuộc tính Hiện thị', 'Hiển thị HP, MP, Sức mạnh, Nhanh nhẹn và danh sách kỹ năng.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO components (id, code, display_name, description) VALUES
  ('018f000a-0000-7000-8000-00000000012e', 'cmp_daily_routine_completion', 'Hoàn thành Lộ trình Thường nhật', 'Nhận hộp quà tân thủ và tăng nhẹ điểm số nền tảng.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO components (id, code, display_name, description) VALUES
  ('018f000a-0000-7000-8000-00000000012f', 'cmp_ticking_countdown_hazard', 'Đồng hồ Đếm ngược Trừng phạt', 'Cảnh báo dịch chuyển vào sa mạc tử thần nếu dám lười biếng.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO components (id, code, display_name, description) VALUES
  ('018f000a-0000-7000-8000-000000000130', 'cmp_golden_roulette_spark', 'Vòng quay Hoàng kim Phát sáng', 'Nhận được skill bị động phá vỡ giới hạn cấp độ.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO components (id, code, display_name, description) VALUES
  ('018f000a-0000-7000-8000-000000000131', 'cmp_admin_privilege_seizure', 'Đoạt Quyền Quản trị viên', 'Bẻ khóa mã nguồn thế giới, tự do điều chỉnh tham số vạn vật.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

-- 6. Table emotional_ranges
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f000a-0000-7000-8000-000000000191', 'emo_disbelieving_shock', 'Kinh ngạc Hoài nghi')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f000a-0000-7000-8000-000000000192', 'emo_dopamine_level_up_rush', 'Hưng phấn Dopamine Tăng cấp')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f000a-0000-7000-8000-000000000193', 'emo_cold_sweat_urgency', 'Đổ mồ hôi hột Nguy cấp')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f000a-0000-7000-8000-000000000194', 'emo_lottery_jackpot_ecstasy', 'Vỡ òa Trúng số Độc đắc')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f000a-0000-7000-8000-000000000195', 'emo_godlike_omnipotence', 'Tự tin Thần thánh')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

-- 7. Table event_blocks & event_block_emotionals
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f000a-0000-7000-8000-0000000001f5', 'eb_sys_awaken', '018f000a-0000-7000-8000-00000000012d', '018f000a-0000-7000-8000-000000000065', '018f000a-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f000a-0000-7000-8000-000000000262', (SELECT id FROM emotional_ranges WHERE code = 'emo_disbelieving_shock'), '018f000a-0000-7000-8000-0000000001f5')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f000a-0000-7000-8000-0000000001f6', 'eb_sys_grind', '018f000a-0000-7000-8000-00000000012e', '018f000a-0000-7000-8000-000000000066', '018f000a-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f000a-0000-7000-8000-00000000026d', (SELECT id FROM emotional_ranges WHERE code = 'emo_dopamine_level_up_rush'), '018f000a-0000-7000-8000-0000000001f6')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f000a-0000-7000-8000-0000000001f7', 'eb_sys_penalty', '018f000a-0000-7000-8000-00000000012f', '018f000a-0000-7000-8000-000000000067', '018f000a-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f000a-0000-7000-8000-000000000278', (SELECT id FROM emotional_ranges WHERE code = 'emo_cold_sweat_urgency'), '018f000a-0000-7000-8000-0000000001f7')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f000a-0000-7000-8000-0000000001f8', 'eb_sys_gacha', '018f000a-0000-7000-8000-000000000130', '018f000a-0000-7000-8000-000000000068', '018f000a-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f000a-0000-7000-8000-000000000283', (SELECT id FROM emotional_ranges WHERE code = 'emo_lottery_jackpot_ecstasy'), '018f000a-0000-7000-8000-0000000001f8')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f000a-0000-7000-8000-0000000001f9', 'eb_sys_transcend', '018f000a-0000-7000-8000-000000000131', '018f000a-0000-7000-8000-000000000069', '018f000a-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f000a-0000-7000-8000-00000000028e', (SELECT id FROM emotional_ranges WHERE code = 'emo_godlike_omnipotence'), '018f000a-0000-7000-8000-0000000001f9')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

-- 8. Table event_transitions
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f000a-0000-7000-8000-0000000002bd', 'Khai mở giao diện cày cấp', '018f000a-0000-7000-8000-0000000001f5', '018f000a-0000-7000-8000-0000000001f6', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f000a-0000-7000-8000-0000000002be', 'Hình phạt treo đầu súng', '018f000a-0000-7000-8000-0000000001f6', '018f000a-0000-7000-8000-0000000001f7', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f000a-0000-7000-8000-0000000002bf', 'Phần thưởng thần thoại đảo chiều', '018f000a-0000-7000-8000-0000000001f7', '018f000a-0000-7000-8000-0000000001f8', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f000a-0000-7000-8000-0000000002c0', 'Xâm nhập căn nguyên hệ thống', '018f000a-0000-7000-8000-0000000001f8', '018f000a-0000-7000-8000-0000000001f9', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

-- 9. Table constraints & motif_constraints
INSERT INTO constraints (id, code, require, reason) VALUES
  ('018f000a-0000-7000-8000-000000000321', 'sys_stat_progression_logic', 'Chỉ số sức mạnh khi tăng phải có tác động định lượng cụ thể lên chiến đấu (tốc độ, sát thương).', 'Duy trì tính thuyết phục của dòng LitRPG đối với độc giả yêu thích dữ liệu.')
  ON CONFLICT (code) DO UPDATE SET require = EXCLUDED.require, reason = EXCLUDED.reason;

INSERT INTO motif_constraints (id, code, constraint_id, motif_id, syntax, constrain) VALUES
  ('018f000a-0000-7000-8000-000000000385', 'he_thong_luu_system_progression_sys_stat_progression_logic', (SELECT id FROM constraints WHERE code = 'sys_stat_progression_logic'), '018f000a-0000-7000-8000-000000000001', 'VALIDATE_MOTIF_RULE(''he_thong_luu_system_progression'', ''sys_stat_progression_logic'')', 'Chỉ số sức mạnh khi tăng phải có tác động định lượng cụ thể lên chiến đấu (tốc độ, sát thương).')
  ON CONFLICT (code) DO UPDATE SET syntax = EXCLUDED.syntax, constrain = EXCLUDED.constrain;

INSERT INTO constraints (id, code, require, reason) VALUES
  ('018f000a-0000-7000-8000-000000000322', 'sys_limit_on_exploit', 'Lỗ hổng hệ thống (exploit) phải đi kèm nguy cơ bị thực thể tạo ra hệ thống truy lùng thanh trừ.', 'Tạo kịch tính và mâu thuẫn lớn hơn ở giai đoạn cuối truyện.')
  ON CONFLICT (code) DO UPDATE SET require = EXCLUDED.require, reason = EXCLUDED.reason;

INSERT INTO motif_constraints (id, code, constraint_id, motif_id, syntax, constrain) VALUES
  ('018f000a-0000-7000-8000-000000000386', 'he_thong_luu_system_progression_sys_limit_on_exploit', (SELECT id FROM constraints WHERE code = 'sys_limit_on_exploit'), '018f000a-0000-7000-8000-000000000001', 'VALIDATE_MOTIF_RULE(''he_thong_luu_system_progression'', ''sys_limit_on_exploit'')', 'Lỗ hổng hệ thống (exploit) phải đi kèm nguy cơ bị thực thể tạo ra hệ thống truy lùng thanh trừ.')
  ON CONFLICT (code) DO UPDATE SET syntax = EXCLUDED.syntax, constrain = EXCLUDED.constrain;

-- Tham chiếu nghiên cứu thị trường: + [TV Tropes - Peggy Sue: Second Chance Time Regression & Revenge Trope](https://tvtropes.org/pmwiki/pmwiki.php/Main/PeggySue):
-- 1. Table motifs
INSERT INTO motifs (id, code, display_name, category, version, is_system, description, min_chapter, max_chapter) VALUES
  ('018f000b-0000-7000-8000-000000000001', 'trong_sinh_bao_thu_regression', 'Trọng sinh Báo thù (Time Regression Revenge Trope)', 'plot', '1.0.0', true, 'Nhân vật kiếp trước tin lầm trà xanh, cặn bã khiến gia tộc tan nát, chết trong uất hận; được trời cho tái sinh về thời điểm then chốt trước khi bi kịch xảy ra, nắm giữ ký ức tương lai để chặn đứng âm mưu và trả thù từng kẻ một.', 35, 250);

-- 2. Table motif_applicable_genres
INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f000b-0000-7000-8000-000000000015', '018f000b-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'trong_sinh'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f000b-0000-7000-8000-000000001015', '018f000b-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'romance'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f000b-0000-7000-8000-000000000017', '018f000b-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'mat_the'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

-- 3. Table state_rules
INSERT INTO state_rules (id, code, display_name) VALUES
  ('018f000b-0000-7000-8000-000000000029', 'future_memory_foreknowledge_rule', 'Quy tắc sử dụng ký ức tương lai làm vũ khí chiến lược')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO state_rules (id, code, display_name) VALUES
  ('018f000b-0000-7000-8000-00000000002a', 'butterfly_effect_divergence', 'Quy tắc hiệu ứng cánh bướm làm biến đổi sự kiện gốc')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

-- 4. Table states & state_down_by_rules
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f000b-0000-7000-8000-000000000065', 'ts_betrayal_and_agony', '1. Kiếp trước Bi thảm & Uất hận (Tragic Past Life Death)', 'Chứng kiến người thân bị sát hại, bản thân bị độc chết hoặc phế truất trong lãnh cung.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f000b-0000-7000-8000-0000000000d2', '018f000b-0000-7000-8000-000000000065', (SELECT id FROM state_rules WHERE code = 'future_memory_foreknowledge_rule'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f000b-0000-7000-8000-000000000066', 'ts_awakening_in_youth', '2. Tỉnh giấc Thời niên thiếu (Rebirth at Crossroads)', 'Mở mắt thấy mình năm 16 tuổi, ngay trước ngày lễ đính ước hoặc ngày xảy ra biến cố khởi đầu.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f000b-0000-7000-8000-0000000000dc', '018f000b-0000-7000-8000-000000000066', (SELECT id FROM state_rules WHERE code = 'future_memory_foreknowledge_rule'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f000b-0000-7000-8000-000000000067', 'ts_preemptive_entrapment', '3. Bố trí Cạm bẫy Phản đòn (Preemptive Scheme Unraveling)', 'Giả vờ ngây thơ, gậy ông đập lưng ông, khiến kẻ hãm hại tự rơi vào chính cái bẫy của chúng.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f000b-0000-7000-8000-0000000000e6', '018f000b-0000-7000-8000-000000000067', (SELECT id FROM state_rules WHERE code = 'future_memory_foreknowledge_rule'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f000b-0000-7000-8000-000000000068', 'ts_stripping_enemy_power', '4. Cắt đứt Vây cánh & Chỗ dựa (Dismantling Enemy Faction)', 'Tước đoạt tài sản, địa vị, bóc trần bộ mặt đạo đức giả của kẻ thù trước toàn thiên hạ.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f000b-0000-7000-8000-0000000000f1', '018f000b-0000-7000-8000-000000000068', (SELECT id FROM state_rules WHERE code = 'butterfly_effect_divergence'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f000b-0000-7000-8000-000000000069', 'ts_rewrite_destiny_peace', '5. Viết lại Vận mệnh & Bình yên (Destiny Rewritten & Catharsis)', 'Bảo vệ trọn vẹn gia đình, kẻ thù nhận bản án thích đáng, bản thân tìm được hạnh phúc đích thực.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f000b-0000-7000-8000-0000000000fa', '018f000b-0000-7000-8000-000000000069', (SELECT id FROM state_rules WHERE code = 'future_memory_foreknowledge_rule'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

-- 5. Table components
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f000b-0000-7000-8000-00000000012d', 'cmp_blood_soaked_reminiscence', 'Ký ức Máu Lãnh cung', 'Lời trăn trối nguyền rủa kẻ phản trắc trước khi nhắm mắt.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO components (id, code, display_name, description) VALUES
  ('018f000b-0000-7000-8000-00000000012e', 'cmp_mirror_youth_reflection', 'Soi gương Nhận ra Thanh xuân', 'Chạm vào làn da chưa vết sẹo, xác định mốc thời gian trùng sinh.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO components (id, code, display_name, description) VALUES
  ('018f000b-0000-7000-8000-00000000012f', 'cmp_poison_wine_switch', 'Hoán đổi Chén rượu Độc', 'Bắt quả tang tại trận kẻ hạ thủ trong đại tiệc gia đình.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO components (id, code, display_name, description) VALUES
  ('018f000b-0000-7000-8000-000000000130', 'cmp_ledger_fraud_exposure', 'Tung Sổ sách Tham ô Công khai', 'Triệt hạ điểm tựa quyền lực của nhị phòng hay gian thần.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO components (id, code, display_name, description) VALUES
  ('018f000b-0000-7000-8000-000000000131', 'cmp_triumphant_wedding_reign', 'Đại lễ Đăng quang / Hôn lễ Viên mãn', 'Nắm tay đồng minh đích thực nhìn non sông tươi đẹp.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

-- 6. Table emotional_ranges
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f000b-0000-7000-8000-000000000191', 'emo_bloodcurdling_hatred', 'Oán hận Ngút trời')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f000b-0000-7000-8000-000000000192', 'emo_shivering_disbelief', 'Bàng hoàng Run rẩy')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f000b-0000-7000-8000-000000000193', 'emo_icy_calculated_malice', 'Lạnh lùng Toan tính')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f000b-0000-7000-8000-000000000194', 'emo_savory_retribution', 'Thỏa mãn Báo thù')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f000b-0000-7000-8000-000000000195', 'emo_profound_tranquility', 'Thanh thản Bình an')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

-- 7. Table event_blocks & event_block_emotionals
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f000b-0000-7000-8000-0000000001f5', 'eb_ts_past', '018f000b-0000-7000-8000-00000000012d', '018f000b-0000-7000-8000-000000000065', '018f000b-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f000b-0000-7000-8000-000000000262', (SELECT id FROM emotional_ranges WHERE code = 'emo_bloodcurdling_hatred'), '018f000b-0000-7000-8000-0000000001f5')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f000b-0000-7000-8000-0000000001f6', 'eb_ts_awaken', '018f000b-0000-7000-8000-00000000012e', '018f000b-0000-7000-8000-000000000066', '018f000b-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f000b-0000-7000-8000-00000000026d', (SELECT id FROM emotional_ranges WHERE code = 'emo_shivering_disbelief'), '018f000b-0000-7000-8000-0000000001f6')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f000b-0000-7000-8000-0000000001f7', 'eb_ts_trap', '018f000b-0000-7000-8000-00000000012f', '018f000b-0000-7000-8000-000000000067', '018f000b-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f000b-0000-7000-8000-000000000278', (SELECT id FROM emotional_ranges WHERE code = 'emo_icy_calculated_malice'), '018f000b-0000-7000-8000-0000000001f7')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f000b-0000-7000-8000-0000000001f8', 'eb_ts_strip', '018f000b-0000-7000-8000-000000000130', '018f000b-0000-7000-8000-000000000068', '018f000b-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f000b-0000-7000-8000-000000000283', (SELECT id FROM emotional_ranges WHERE code = 'emo_savory_retribution'), '018f000b-0000-7000-8000-0000000001f8')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f000b-0000-7000-8000-0000000001f9', 'eb_ts_peace', '018f000b-0000-7000-8000-000000000131', '018f000b-0000-7000-8000-000000000069', '018f000b-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f000b-0000-7000-8000-00000000028e', (SELECT id FROM emotional_ranges WHERE code = 'emo_profound_tranquility'), '018f000b-0000-7000-8000-0000000001f9')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

-- 8. Table event_transitions
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f000b-0000-7000-8000-0000000002bd', 'Mở mắt hồi quy thời niên thiếu', '018f000b-0000-7000-8000-0000000001f5', '018f000b-0000-7000-8000-0000000001f6', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f000b-0000-7000-8000-0000000002be', 'Lập mưu phản kích kẻ ác', '018f000b-0000-7000-8000-0000000001f6', '018f000b-0000-7000-8000-0000000001f7', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f000b-0000-7000-8000-0000000002bf', 'Đập tan vây cánh tập đoàn đối địch', '018f000b-0000-7000-8000-0000000001f7', '018f000b-0000-7000-8000-0000000001f8', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f000b-0000-7000-8000-0000000002c0', 'Khép lại oán thù xây đắp tương lai', '018f000b-0000-7000-8000-0000000001f8', '018f000b-0000-7000-8000-0000000001f9', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

-- 9. Table constraints & motif_constraints
INSERT INTO constraints (id, code, require, reason) VALUES
  ('018f000b-0000-7000-8000-000000000321', 'ts_butterfly_divergence_complication', 'Hành động can thiệp của nhân vật chính bắt buộc phải tạo ra biến số mới ngoài ký ức kiếp trước.', 'Tránh cốt truyện bị một màu dễ dãi, duy trì sự bất ngờ kịch tính.')
  ON CONFLICT (code) DO UPDATE SET require = EXCLUDED.require, reason = EXCLUDED.reason;

INSERT INTO motif_constraints (id, code, constraint_id, motif_id, syntax, constrain) VALUES
  ('018f000b-0000-7000-8000-000000000385', 'trong_sinh_bao_thu_regression_ts_butterfly_divergence_complication', (SELECT id FROM constraints WHERE code = 'ts_butterfly_divergence_complication'), '018f000b-0000-7000-8000-000000000001', 'VALIDATE_MOTIF_RULE(''trong_sinh_bao_thu_regression'', ''ts_butterfly_divergence_complication'')', 'Hành động can thiệp của nhân vật chính bắt buộc phải tạo ra biến số mới ngoài ký ức kiếp trước.')
  ON CONFLICT (code) DO UPDATE SET syntax = EXCLUDED.syntax, constrain = EXCLUDED.constrain;

INSERT INTO constraints (id, code, require, reason) VALUES
  ('018f000b-0000-7000-8000-000000000322', 'ts_no_premature_forgiveness', 'Nhân vật chính tuyệt đối không được tha thứ cho kẻ thù kiếp trước chỉ vì kiếp này chúng chưa kịp ra tay.', 'Giữ vững cảm xúc chính thống của thể loại sảng khoái phục thù.')
  ON CONFLICT (code) DO UPDATE SET require = EXCLUDED.require, reason = EXCLUDED.reason;

INSERT INTO motif_constraints (id, code, constraint_id, motif_id, syntax, constrain) VALUES
  ('018f000b-0000-7000-8000-000000000386', 'trong_sinh_bao_thu_regression_ts_no_premature_forgiveness', (SELECT id FROM constraints WHERE code = 'ts_no_premature_forgiveness'), '018f000b-0000-7000-8000-000000000001', 'VALIDATE_MOTIF_RULE(''trong_sinh_bao_thu_regression'', ''ts_no_premature_forgiveness'')', 'Nhân vật chính tuyệt đối không được tha thứ cho kẻ thù kiếp trước chỉ vì kiếp này chúng chưa kịp ra tay.')
  ON CONFLICT (code) DO UPDATE SET syntax = EXCLUDED.syntax, constrain = EXCLUDED.constrain;

-- Tham chiếu nghiên cứu thị trường: + [Helping Writers Become Authors - How to Write Character Arcs: Positive Change and Flat Arcs](https://www.helpingwritersbecomeauthors.com/write-character-arcs/):
-- 1. Table motifs
INSERT INTO motifs (id, code, display_name, category, version, is_system, description, min_chapter, max_chapter) VALUES
  ('018f000c-0000-7000-8000-000000000001', 'positive_change_character_arc', 'Cung Phát triển Tính cách Tích cực (Positive Change Arc)', 'character_arc', '1.0.0', true, 'Lý thuyết cốt lõi của K.M. Weiland về cung nhân vật tích cực: Nhân vật khởi đầu bằng niềm tin mù quáng vào Ngụy ngôn (The Lie). Qua thử thách, họ nhận ra Sự thật (The Truth), buông bỏ cái Tôi vị kỷ (Want) để ôm trọn Nhu cầu đích thực (Need).', 20, 100);

-- 2. Table motif_applicable_genres
INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f000c-0000-7000-8000-000000000015', '018f000c-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'literary_fiction'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f000c-0000-7000-8000-000000000016', '018f000c-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'contemporary_fiction'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f000c-0000-7000-8000-000000000017', '018f000c-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'fantasy'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

-- 3. Table state_rules
INSERT INTO state_rules (id, code, display_name) VALUES
  ('018f000c-0000-7000-8000-000000000029', 'lie_vs_truth_internal_dialectic', 'Quy tắc biện chứng nội tâm giữa Ngụy ngôn và Chân lý')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO state_rules (id, code, display_name) VALUES
  ('018f000c-0000-7000-8000-00000000002a', 'want_subjugation_to_need', 'Quy tắc hy sinh điều nhân vật Muốn để đạt được điều nhân vật Cần')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

-- 4. Table states & state_down_by_rules
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f000c-0000-7000-8000-000000000065', 'pca_living_the_lie', '1. Sống trong Ngụy ngôn (Living the Lie)', 'Nhân vật tin vào một quan niệm sai lầm về bản thân hoặc thế giới để tự bảo vệ vết thương cũ (The Ghost).')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f000c-0000-7000-8000-0000000000d2', '018f000c-0000-7000-8000-000000000065', (SELECT id FROM state_rules WHERE code = 'lie_vs_truth_internal_dialectic'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f000c-0000-7000-8000-000000000066', 'pca_first_glimpse_of_truth', '2. Thoáng thấy Chân lý (First Glimpse of the Truth)', 'Gặp gỡ một tấm gương hoặc trải nghiệm thách thức niềm tin cũ, hé mở con đường mới.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f000c-0000-7000-8000-0000000000dc', '018f000c-0000-7000-8000-000000000066', (SELECT id FROM state_rules WHERE code = 'lie_vs_truth_internal_dialectic'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f000c-0000-7000-8000-000000000067', 'pca_clinging_to_comfort_lie', '3. Bám chấp Vỏ ốc Ngụy ngôn (Resisting Truth & Clinging to Lie)', 'Khi áp lực tăng cao, nhân vật cố chấp dùng lại thói quen cũ nhưng thất bại thảm hại.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f000c-0000-7000-8000-0000000000e7', '018f000c-0000-7000-8000-000000000067', (SELECT id FROM state_rules WHERE code = 'want_subjugation_to_need'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f000c-0000-7000-8000-000000000068', 'pca_surrendering_the_want', '4. Buông bỏ Tham vọng Vị kỷ (Surrendering the Want for the Need)', 'Khoảnh khắc nhận thức đau đớn: chấp nhận từ bỏ mục tiêu ích kỷ để bảo vệ giá trị cao đẹp.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f000c-0000-7000-8000-0000000000f1', '018f000c-0000-7000-8000-000000000068', (SELECT id FROM state_rules WHERE code = 'want_subjugation_to_need'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f000c-0000-7000-8000-000000000069', 'pca_embodying_the_truth', '5. Hiện thân của Chân lý (Embodying the Truth in Climax)', 'Vượt qua thử thách tối thượng nhờ sức mạnh của sự thật, hoàn thiện nhân cách trọn vẹn.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f000c-0000-7000-8000-0000000000fa', '018f000c-0000-7000-8000-000000000069', (SELECT id FROM state_rules WHERE code = 'lie_vs_truth_internal_dialectic'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

-- 5. Table components
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f000c-0000-7000-8000-00000000012d', 'cmp_defensive_flaw_display', 'Bộc lộ Khiếm khuyết Phòng thủ', 'Nhân vật đẩy người khác ra xa vì sợ bị tổn thương lần nữa.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO components (id, code, display_name, description) VALUES
  ('018f000c-0000-7000-8000-00000000012e', 'cmp_unsettling_altruism_witness', 'Chứng kiến Hành vi Cao thượng', 'Một người bạn hy sinh vô điều kiện khiến nhân vật lung lay.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO components (id, code, display_name, description) VALUES
  ('018f000c-0000-7000-8000-00000000012f', 'cmp_catastrophic_lie_failure', 'Thất bại Thảm hại của Ngụy ngôn', 'Kế hoạch dựa trên sự dối trá sụp đổ làm tổn thương đồng đội.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO components (id, code, display_name, description) VALUES
  ('018f000c-0000-7000-8000-000000000130', 'cmp_tearful_ego_sacrifice', 'Hiến tế Cái tôi trong Nước mắt', 'Tự tay ném bỏ vương miện hoặc danh vọng hão huyền.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO components (id, code, display_name, description) VALUES
  ('018f000c-0000-7000-8000-000000000131', 'cmp_integrated_courage_triumph', 'Chiến thắng Từ Tâm thế Hợp nhất', 'Hành động vì người khác mang lại sức mạnh bất khả chiến bại.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

-- 6. Table emotional_ranges
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f000c-0000-7000-8000-000000000191', 'emo_guarded_cynicism', 'Hoài nghi Đề phòng')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f000c-0000-7000-8000-000000000192', 'emo_flicker_of_hope', 'Tia sáng Hy vọng Mong manh')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f000c-0000-7000-8000-000000000193', 'emo_defensive_stubbornness', 'Cố chấp Ngoan cố')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f000c-0000-7000-8000-000000000194', 'emo_heartbreaking_catharsis', 'Nghẹn ngào Giác ngộ')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f000c-0000-7000-8000-000000000195', 'emo_sovereign_wholeness', 'Toàn vẹn Tự do')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

-- 7. Table event_blocks & event_block_emotionals
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f000c-0000-7000-8000-0000000001f5', 'eb_pca_lie', '018f000c-0000-7000-8000-00000000012d', '018f000c-0000-7000-8000-000000000065', '018f000c-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f000c-0000-7000-8000-000000000262', (SELECT id FROM emotional_ranges WHERE code = 'emo_guarded_cynicism'), '018f000c-0000-7000-8000-0000000001f5')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f000c-0000-7000-8000-0000000001f6', 'eb_pca_glimpse', '018f000c-0000-7000-8000-00000000012e', '018f000c-0000-7000-8000-000000000066', '018f000c-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f000c-0000-7000-8000-00000000026d', (SELECT id FROM emotional_ranges WHERE code = 'emo_flicker_of_hope'), '018f000c-0000-7000-8000-0000000001f6')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f000c-0000-7000-8000-0000000001f7', 'eb_pca_resist', '018f000c-0000-7000-8000-00000000012f', '018f000c-0000-7000-8000-000000000067', '018f000c-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f000c-0000-7000-8000-000000000278', (SELECT id FROM emotional_ranges WHERE code = 'emo_defensive_stubbornness'), '018f000c-0000-7000-8000-0000000001f7')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f000c-0000-7000-8000-0000000001f8', 'eb_pca_surrender', '018f000c-0000-7000-8000-000000000130', '018f000c-0000-7000-8000-000000000068', '018f000c-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f000c-0000-7000-8000-000000000283', (SELECT id FROM emotional_ranges WHERE code = 'emo_heartbreaking_catharsis'), '018f000c-0000-7000-8000-0000000001f8')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f000c-0000-7000-8000-0000000001f9', 'eb_pca_truth', '018f000c-0000-7000-8000-000000000131', '018f000c-0000-7000-8000-000000000069', '018f000c-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f000c-0000-7000-8000-00000000028e', (SELECT id FROM emotional_ranges WHERE code = 'emo_sovereign_wholeness'), '018f000c-0000-7000-8000-0000000001f9')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

-- 8. Table event_transitions
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f000c-0000-7000-8000-0000000002bd', 'Hé mở chân trời mới', '018f000c-0000-7000-8000-0000000001f5', '018f000c-0000-7000-8000-0000000001f6', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f000c-0000-7000-8000-0000000002be', 'Khủng hoảng níu kéo ảo ảnh', '018f000c-0000-7000-8000-0000000001f6', '018f000c-0000-7000-8000-0000000001f7', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f000c-0000-7000-8000-0000000002bf', 'Buông tay đầu hàng sự thật', '018f000c-0000-7000-8000-0000000001f7', '018f000c-0000-7000-8000-0000000001f8', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f000c-0000-7000-8000-0000000002c0', 'Trỗi dậy với tâm thức hoàn thiện', '018f000c-0000-7000-8000-0000000001f8', '018f000c-0000-7000-8000-0000000001f9', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

-- 9. Table constraints & motif_constraints
INSERT INTO constraints (id, code, require, reason) VALUES
  ('018f000c-0000-7000-8000-000000000321', 'pca_ghost_wound_necessity', 'Ngụy ngôn (The Lie) phải bắt nguồn từ một vết thương tâm lý quá khứ (The Ghost/Wound) cụ thể.', 'Giữ tính logic tâm lý chiều sâu, giải thích lý do nhân vật bảo thủ.')
  ON CONFLICT (code) DO UPDATE SET require = EXCLUDED.require, reason = EXCLUDED.reason;

INSERT INTO motif_constraints (id, code, constraint_id, motif_id, syntax, constrain) VALUES
  ('018f000c-0000-7000-8000-000000000385', 'positive_change_character_arc_pca_ghost_wound_necessity', (SELECT id FROM constraints WHERE code = 'pca_ghost_wound_necessity'), '018f000c-0000-7000-8000-000000000001', 'VALIDATE_MOTIF_RULE(''positive_change_character_arc'', ''pca_ghost_wound_necessity'')', 'Ngụy ngôn (The Lie) phải bắt nguồn từ một vết thương tâm lý quá khứ (The Ghost/Wound) cụ thể.')
  ON CONFLICT (code) DO UPDATE SET syntax = EXCLUDED.syntax, constrain = EXCLUDED.constrain;

INSERT INTO constraints (id, code, require, reason) VALUES
  ('018f000c-0000-7000-8000-000000000322', 'pca_need_trumps_want', 'Tại cao trào, chiến thắng chỉ đạt được khi nhân vật ưu tiên Nhu cầu tinh thần (Need) hơn Tham muốn vật chất (Want).', 'Bản chất định nghĩa cung phát triển nhân vật tích cực của K.M. Weiland.')
  ON CONFLICT (code) DO UPDATE SET require = EXCLUDED.require, reason = EXCLUDED.reason;

INSERT INTO motif_constraints (id, code, constraint_id, motif_id, syntax, constrain) VALUES
  ('018f000c-0000-7000-8000-000000000386', 'positive_change_character_arc_pca_need_trumps_want', (SELECT id FROM constraints WHERE code = 'pca_need_trumps_want'), '018f000c-0000-7000-8000-000000000001', 'VALIDATE_MOTIF_RULE(''positive_change_character_arc'', ''pca_need_trumps_want'')', 'Tại cao trào, chiến thắng chỉ đạt được khi nhân vật ưu tiên Nhu cầu tinh thần (Need) hơn Tham muốn vật chất (Want).')
  ON CONFLICT (code) DO UPDATE SET syntax = EXCLUDED.syntax, constrain = EXCLUDED.constrain;

-- Tham chiếu nghiên cứu thị trường: + [Helping Writers Become Authors - How to Write a Negative Character Arc: Disillusionment, Fall, and Corruption](https://www.helpingwritersbecomeauthors.com/negative-character-arc-1/):
-- 1. Table motifs
INSERT INTO motifs (id, code, display_name, category, version, is_system, description, min_chapter, max_chapter) VALUES
  ('018f000d-0000-7000-8000-000000000001', 'corruption_negative_character_arc', 'Cung Nhân vật Tha hóa / Hắc hóa (The Corruption Arc)', 'character_arc', '1.0.0', true, 'Bi kịch tha hóa đạo đức kinh điển: Nhân vật ban đầu nhận biết Sự thật (The Truth) nhưng trước áp lực tàn khốc và cám dỗ quyền lực, họ dần thỏa hiệp, ôm lấy Ngụy ngôn (The Lie) và biến chất thành kẻ phản diện đáng sợ nhất.', 25, 110);

-- 2. Table motif_applicable_genres
INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f000d-0000-7000-8000-000000000015', '018f000d-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'dark'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f000d-0000-7000-8000-000000001016', '018f000d-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'psychological'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

-- 3. Table state_rules
INSERT INTO state_rules (id, code, display_name) VALUES
  ('018f000d-0000-7000-8000-000000000029', 'moral_rationalization_progression', 'Quy tắc hợp lý hóa tội lỗi từng bậc thang')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO state_rules (id, code, display_name) VALUES
  ('018f000d-0000-7000-8000-00000000002a', 'abyss_irreversibility_point', 'Quy tắc điểm không thể quay đầu của sự tha hóa')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

-- 4. Table states & state_down_by_rules
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f000d-0000-7000-8000-000000000065', 'cna_noble_idealism', '1. Lý tưởng Cao thượng Sơ khởi (Noble Idealism & Known Truth)', 'Nhân vật kiên định bảo vệ chính nghĩa, tin vào lòng tốt và các giá trị đạo đức mẫu mực.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f000d-0000-7000-8000-0000000000d2', '018f000d-0000-7000-8000-000000000065', (SELECT id FROM state_rules WHERE code = 'moral_rationalization_progression'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f000d-0000-7000-8000-000000000066', 'cna_impotence_against_evil', '2. Bất lực trước Cường quyền / Bi kịch (Impotence in Crisis)', 'Quy tắc chính trực bị kẻ thủ ác chà đạp; chứng kiến người thương yêu chịu đau đớn mà luật lệ không cứu được.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f000d-0000-7000-8000-0000000000dc', '018f000d-0000-7000-8000-000000000066', (SELECT id FROM state_rules WHERE code = 'moral_rationalization_progression'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f000d-0000-7000-8000-000000000067', 'cna_first_dark_compromise', '3. Thỏa hiệp Đen tối Đầu tiên (First Moral Compromise)', 'Sử dụng biện pháp tàn bạo vì ''mục đích tốt đẹp''; bắt đầu tin rằng ''mục đích biện minh cho phương tiện''.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f000d-0000-7000-8000-0000000000e6', '018f000d-0000-7000-8000-000000000067', (SELECT id FROM state_rules WHERE code = 'moral_rationalization_progression'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f000d-0000-7000-8000-000000000068', 'cna_point_of_no_return_atrocity', '4. Điểm Vực thẳm / Tội ác Không thể Tha thứ (Crossing Point of No Return)', 'Hiến tế đồng đội cũ hoặc phản bội lý tưởng cốt lõi để đổi lấy quyền năng tuyệt đối.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f000d-0000-7000-8000-0000000000f1', '018f000d-0000-7000-8000-000000000068', (SELECT id FROM state_rules WHERE code = 'abyss_irreversibility_point'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f000d-0000-7000-8000-000000000069', 'cna_complete_tyrannical_decay', '5. Tha hóa Toàn diện / Trở thành Bạo chúa (Complete Decay into Villainy)', 'Hoàn toàn bị nuốt chửng bởi ảo tưởng kiểm soát; tàn sát bất kỳ ai cản đường nhân danh trật tự mới.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f000d-0000-7000-8000-0000000000fb', '018f000d-0000-7000-8000-000000000069', (SELECT id FROM state_rules WHERE code = 'abyss_irreversibility_point'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

-- 5. Table components
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f000d-0000-7000-8000-00000000012d', 'cmp_shining_banner_stand', 'Lời thề Dưới Quân kỳ', 'Khẳng định danh dự hiệp sĩ và công lý tuyệt đối.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO components (id, code, display_name, description) VALUES
  ('018f000d-0000-7000-8000-00000000012e', 'cmp_unpunished_atrocity_witness', 'Chứng kiến Ác nhân Vô tội vạ', 'Hệ thống pháp luật thối nát tha bổng kẻ thủ ác tàn nhẫn.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO components (id, code, display_name, description) VALUES
  ('018f000d-0000-7000-8000-00000000012f', 'cmp_forbidden_art_usage', 'Khai triển Cấm thuật Cứu viện', 'Hút linh hồn kẻ thù để bảo vệ thành trì.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO components (id, code, display_name, description) VALUES
  ('018f000d-0000-7000-8000-000000000130', 'cmp_slaying_the_moral_anchor', 'Hạ sát Điểm tựa Đạo đức', 'Tự tay sát hại người thầy hoặc người bạn từng khuyên can mình.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO components (id, code, display_name, description) VALUES
  ('018f000d-0000-7000-8000-000000000131', 'cmp_obsidian_throne_coronation', 'Đăng cơ trên Ngai vàng Hắc ám', 'Khoác lên mình áo choàng bạo chúa với đôi mắt lạnh lùng vô cảm.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

-- 6. Table emotional_ranges
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f000d-0000-7000-8000-000000000191', 'emo_pure_righteousness', 'Chính nghĩa Trong sáng')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f000d-0000-7000-8000-000000000192', 'emo_burning_helplessness', 'Bất lực Bùng cháy')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f000d-0000-7000-8000-000000000193', 'emo_sinister_rationalization', 'Biện minh U ám')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f000d-0000-7000-8000-000000000194', 'emo_shattered_remorse', 'Hối hận Vỡ vụn Câm lặng')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f000d-0000-7000-8000-000000000195', 'emo_chilling_megalomania', 'Cuồng vọng Lạnh giá')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

-- 7. Table event_blocks & event_block_emotionals
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f000d-0000-7000-8000-0000000001f5', 'eb_cna_noble', '018f000d-0000-7000-8000-00000000012d', '018f000d-0000-7000-8000-000000000065', '018f000d-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f000d-0000-7000-8000-000000000262', (SELECT id FROM emotional_ranges WHERE code = 'emo_pure_righteousness'), '018f000d-0000-7000-8000-0000000001f5')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f000d-0000-7000-8000-0000000001f6', 'eb_cna_impotent', '018f000d-0000-7000-8000-00000000012e', '018f000d-0000-7000-8000-000000000066', '018f000d-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f000d-0000-7000-8000-00000000026d', (SELECT id FROM emotional_ranges WHERE code = 'emo_burning_helplessness'), '018f000d-0000-7000-8000-0000000001f6')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f000d-0000-7000-8000-0000000001f7', 'eb_cna_compromise', '018f000d-0000-7000-8000-00000000012f', '018f000d-0000-7000-8000-000000000067', '018f000d-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f000d-0000-7000-8000-000000000278', (SELECT id FROM emotional_ranges WHERE code = 'emo_sinister_rationalization'), '018f000d-0000-7000-8000-0000000001f7')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f000d-0000-7000-8000-0000000001f8', 'eb_cna_abyss', '018f000d-0000-7000-8000-000000000130', '018f000d-0000-7000-8000-000000000068', '018f000d-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f000d-0000-7000-8000-000000000283', (SELECT id FROM emotional_ranges WHERE code = 'emo_shattered_remorse'), '018f000d-0000-7000-8000-0000000001f8')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f000d-0000-7000-8000-0000000001f9', 'eb_cna_decay', '018f000d-0000-7000-8000-000000000131', '018f000d-0000-7000-8000-000000000069', '018f000d-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f000d-0000-7000-8000-00000000028e', (SELECT id FROM emotional_ranges WHERE code = 'emo_chilling_megalomania'), '018f000d-0000-7000-8000-0000000001f9')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

-- 8. Table event_transitions
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f000d-0000-7000-8000-0000000002bd', 'Vỡ mộng trước hiện thực tàn khốc', '018f000d-0000-7000-8000-0000000001f5', '018f000d-0000-7000-8000-0000000001f6', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f000d-0000-7000-8000-0000000002be', 'Bước qua lằn ranh đạo đức đầu tiên', '018f000d-0000-7000-8000-0000000001f6', '018f000d-0000-7000-8000-0000000001f7', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f000d-0000-7000-8000-0000000002bf', 'Sa lầy vào tội ác không thể tha thứ', '018f000d-0000-7000-8000-0000000001f7', '018f000d-0000-7000-8000-0000000001f8', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f000d-0000-7000-8000-0000000002c0', 'Biến chất thành ma đầu tàn bạo', '018f000d-0000-7000-8000-0000000001f8', '018f000d-0000-7000-8000-0000000001f9', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

-- 9. Table constraints & motif_constraints
INSERT INTO constraints (id, code, require, reason) VALUES
  ('018f000d-0000-7000-8000-000000000321', 'cna_sympathetic_initial_intent', 'Động cơ ban đầu của nhân vật bắt buộc phải mang tính cao đẹp và vì người khác (cứu gia đình, chấm dứt chiến tranh).', 'Bi kịch tha hóa chỉ có sức lay động khi người đọc cảm thương cho lý do vấp ngã.')
  ON CONFLICT (code) DO UPDATE SET require = EXCLUDED.require, reason = EXCLUDED.reason;

INSERT INTO motif_constraints (id, code, constraint_id, motif_id, syntax, constrain) VALUES
  ('018f000d-0000-7000-8000-000000000385', 'corruption_negative_character_arc_cna_sympathetic_initial_intent', (SELECT id FROM constraints WHERE code = 'cna_sympathetic_initial_intent'), '018f000d-0000-7000-8000-000000000001', 'VALIDATE_MOTIF_RULE(''corruption_negative_character_arc'', ''cna_sympathetic_initial_intent'')', 'Động cơ ban đầu của nhân vật bắt buộc phải mang tính cao đẹp và vì người khác (cứu gia đình, chấm dứt chiến tranh).')
  ON CONFLICT (code) DO UPDATE SET syntax = EXCLUDED.syntax, constrain = EXCLUDED.constrain;

INSERT INTO constraints (id, code, require, reason) VALUES
  ('018f000d-0000-7000-8000-000000000322', 'cna_severing_moral_anchor', 'Trước khi hắc hóa hoàn toàn, nhân vật phải cắt đứt hoặc sát hại người đóng vai trò là mỏ neo đạo đức (Moral Anchor).', 'Đánh dấu sự sụp đổ vĩnh viễn của tiếng nói lương tâm.')
  ON CONFLICT (code) DO UPDATE SET require = EXCLUDED.require, reason = EXCLUDED.reason;

INSERT INTO motif_constraints (id, code, constraint_id, motif_id, syntax, constrain) VALUES
  ('018f000d-0000-7000-8000-000000000386', 'corruption_negative_character_arc_cna_severing_moral_anchor', (SELECT id FROM constraints WHERE code = 'cna_severing_moral_anchor'), '018f000d-0000-7000-8000-000000000001', 'VALIDATE_MOTIF_RULE(''corruption_negative_character_arc'', ''cna_severing_moral_anchor'')', 'Trước khi hắc hóa hoàn toàn, nhân vật phải cắt đứt hoặc sát hại người đóng vai trò là mỏ neo đạo đức (Moral Anchor).')
  ON CONFLICT (code) DO UPDATE SET syntax = EXCLUDED.syntax, constrain = EXCLUDED.constrain;

-- Tham chiếu nghiên cứu thị trường: + [AI Write Book - Enemies to Lovers: Beat Sheet, Tension Pacing and Emotional Trajectory](https://aiwritebook.com/en/blog/enemies-to-lovers-guide/):
-- 1. Table motifs
INSERT INTO motifs (id, code, display_name, category, version, is_system, description, min_chapter, max_chapter) VALUES
  ('018f000e-0000-7000-8000-000000000001', 'enemies_to_lovers_progression', 'Từ Kẻ thù hóa Người yêu (Enemies to Lovers Dynamic)', 'relationship', '1.0.0', true, 'Mô-típ lãng mạn kinh điển: Hai nhân vật xuất phát điểm ở hai đầu chiến tuyến, khinh ghét lẫn nhau; bị hoàn cảnh cưỡng chế ép buộc hợp tác (Forced Proximity), dần bóc trần điểm yếu nội tâm, biến thù hận thành khao khát nồng cháy và tình yêu bất diệt.', 20, 90);

-- 2. Table motif_applicable_genres
INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f000e-0000-7000-8000-000000000015', '018f000e-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'romance'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

-- 3. Table state_rules
INSERT INTO state_rules (id, code, display_name) VALUES
  ('018f000e-0000-7000-8000-000000000029', 'forced_proximity_catalyst_rule', 'Quy tắc không gian cưỡng chế buộc phải tiếp xúc thân mật')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO state_rules (id, code, display_name) VALUES
  ('018f000e-0000-7000-8000-00000000002a', 'vulnerability_revelation_sequence', 'Quy tắc hé lộ tổn thương trước khi nảy sinh rung cảm')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

-- 4. Table states & state_down_by_rules
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f000e-0000-7000-8000-000000000065', 'e2l_initial_hostility_clash', '1. Thù địch Đối đầu Tuyệt đối (Initial Mutual Hostility)', 'Coi nhau như cái gai trong mắt, đấu khẩu gay gắt, sẵn sàng rút kiếm hoặc triệt hạ nhau trong công việc.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f000e-0000-7000-8000-0000000000d2', '018f000e-0000-7000-8000-000000000065', (SELECT id FROM state_rules WHERE code = 'forced_proximity_catalyst_rule'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f000e-0000-7000-8000-000000000066', 'e2l_forced_proximity_containment', '2. Cưỡng chế Giam cầm / Chung chiến hào (Forced Proximity)', 'Bị nhốt chung hầm ngục, lạc trong bão tuyết, hoặc bị ép kết hôn chính trị; không thể tránh né nhau.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f000e-0000-7000-8000-0000000000dc', '018f000e-0000-7000-8000-000000000066', (SELECT id FROM state_rules WHERE code = 'forced_proximity_catalyst_rule'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f000e-0000-7000-8000-000000000067', 'e2l_unmasking_vulnerability', '3. Hé lộ Điểm yếu & Tôn trọng Miễn cưỡng (Unmasking the Wound)', 'Bắt gặp đối phương băng bó vết thương trong đêm, thấu hiểu bi kịch đằng sau lớp vỏ sắc nhọn.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f000e-0000-7000-8000-0000000000e7', '018f000e-0000-7000-8000-000000000067', (SELECT id FROM state_rules WHERE code = 'vulnerability_revelation_sequence'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f000e-0000-7000-8000-000000000068', 'e2l_smoldering_romantic_tension', '4. Điện thế Lãng mạn Bùng cháy (Smoldering Tension & First Kiss)', 'Sự va chạm thể xác vô tình khơi dậy đam mê ngột ngạt; nụ hôn đầu tiên vừa cuồng nhiệt vừa phòng thủ.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f000e-0000-7000-8000-0000000000f1', '018f000e-0000-7000-8000-000000000068', (SELECT id FROM state_rules WHERE code = 'vulnerability_revelation_sequence'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f000e-0000-7000-8000-000000000069', 'e2l_apparent_betrayal_crisis', '5. Khủng hoảng Hiểu lầm Phản trắc (The Apparent Betrayal / Dark Night)', 'Một bí mật cũ bị khui ra khiến một bên ngỡ rằng mình bị lợi dụng; thù hận cũ bùng phát trở lại.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f000e-0000-7000-8000-0000000000fb', '018f000e-0000-7000-8000-000000000069', (SELECT id FROM state_rules WHERE code = 'vulnerability_revelation_sequence'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f000e-0000-7000-8000-00000000006a', 'e2l_all_in_vow_union', '6. Thú nhận Chân tình & Liên minh Vĩnh hằng (Grand Gesture & Union)', 'Sẵn sàng hy sinh sinh mạng để chứng minh tình cảm, phá vỡ định kiến hai phe, gắn kết trọn đời.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f000e-0000-7000-8000-000000000105', '018f000e-0000-7000-8000-00000000006a', (SELECT id FROM state_rules WHERE code = 'vulnerability_revelation_sequence'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

-- 5. Table components
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f000e-0000-7000-8000-00000000012d', 'cmp_blade_to_throat_dialogue', 'Kề Kiếm vào Cổ Đấu khẩu', 'Cuộc chạm trán nảy lửa với ánh mắt tóe lửa căm hờn.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO components (id, code, display_name, description) VALUES
  ('018f000e-0000-7000-8000-00000000012e', 'cmp_single_bed_storm_shelter', 'Trú bão Chỉ có Một Giường', 'Hoàn cảnh éo le buộc hai người phải chia sẻ thân nhiệt.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO components (id, code, display_name, description) VALUES
  ('018f000e-0000-7000-8000-00000000012f', 'cmp_tending_battle_wounds', 'Băng bó Vết thương Đêm khuya', 'Những ngón tay run rẩy lau sạch vết máu trên lưng trần.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO components (id, code, display_name, description) VALUES
  ('018f000e-0000-7000-8000-000000000130', 'cmp_furious_breathless_kiss', 'Nụ hôn Giận dữ Cuồng nhiệt', 'Không thể kìm nén khao khát sau khi cùng thoát hiểm.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO components (id, code, display_name, description) VALUES
  ('018f000e-0000-7000-8000-000000000131', 'cmp_stolen_military_map_blame', 'Nghi án Bản đồ Bị đánh cắp', 'Vật chứng giả mạo tố cáo đối phương là nội gián.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO components (id, code, display_name, description) VALUES
  ('018f000e-0000-7000-8000-000000000132', 'cmp_defying_both_armies_declaration', 'Tuyên ngôn Tình yêu Trước Hai Trận tuyến', 'Đứng chắn trước mũi tên của đồng minh để bảo vệ người thương.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

-- 6. Table emotional_ranges
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f000e-0000-7000-8000-000000000191', 'emo_vitriolic_scorn', 'Khinh bỉ Cay độc')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f000e-0000-7000-8000-000000000192', 'emo_claustrophobic_friction', 'Bức bối Ngột ngạt')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f000e-0000-7000-8000-000000000193', 'emo_reluctant_compassion', 'Thương cảm Miễn cưỡng')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f000e-0000-7000-8000-000000000194', 'emo_electric_yearning', 'Rạo rực Bỏng cháy')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f000e-0000-7000-8000-000000000195', 'emo_devastating_grief', 'Tan vỡ Đau đớn')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f000e-0000-7000-8000-000000000196', 'emo_unshakable_devotion', 'Thủy chung Bất diệt')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

-- 7. Table event_blocks & event_block_emotionals
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f000e-0000-7000-8000-0000000001f5', 'eb_e2l_hostile', '018f000e-0000-7000-8000-00000000012d', '018f000e-0000-7000-8000-000000000065', '018f000e-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f000e-0000-7000-8000-000000000262', (SELECT id FROM emotional_ranges WHERE code = 'emo_vitriolic_scorn'), '018f000e-0000-7000-8000-0000000001f5')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f000e-0000-7000-8000-0000000001f6', 'eb_e2l_proximity', '018f000e-0000-7000-8000-00000000012e', '018f000e-0000-7000-8000-000000000066', '018f000e-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f000e-0000-7000-8000-00000000026d', (SELECT id FROM emotional_ranges WHERE code = 'emo_claustrophobic_friction'), '018f000e-0000-7000-8000-0000000001f6')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f000e-0000-7000-8000-0000000001f7', 'eb_e2l_wound', '018f000e-0000-7000-8000-00000000012f', '018f000e-0000-7000-8000-000000000067', '018f000e-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f000e-0000-7000-8000-000000000278', (SELECT id FROM emotional_ranges WHERE code = 'emo_reluctant_compassion'), '018f000e-0000-7000-8000-0000000001f7')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f000e-0000-7000-8000-0000000001f8', 'eb_e2l_kiss', '018f000e-0000-7000-8000-000000000130', '018f000e-0000-7000-8000-000000000068', '018f000e-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f000e-0000-7000-8000-000000000283', (SELECT id FROM emotional_ranges WHERE code = 'emo_electric_yearning'), '018f000e-0000-7000-8000-0000000001f8')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f000e-0000-7000-8000-0000000001f9', 'eb_e2l_betray', '018f000e-0000-7000-8000-000000000131', '018f000e-0000-7000-8000-000000000069', '018f000e-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f000e-0000-7000-8000-00000000028e', (SELECT id FROM emotional_ranges WHERE code = 'emo_devastating_grief'), '018f000e-0000-7000-8000-0000000001f9')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f000e-0000-7000-8000-0000000001fa', 'eb_e2l_union', '018f000e-0000-7000-8000-000000000132', '018f000e-0000-7000-8000-00000000006a', '018f000e-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f000e-0000-7000-8000-000000000299', (SELECT id FROM emotional_ranges WHERE code = 'emo_unshakable_devotion'), '018f000e-0000-7000-8000-0000000001fa')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

-- 8. Table event_transitions
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f000e-0000-7000-8000-0000000002bd', 'Dồn vào tình thế kẹt chung', '018f000e-0000-7000-8000-0000000001f5', '018f000e-0000-7000-8000-0000000001f6', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f000e-0000-7000-8000-0000000002be', 'Lột bỏ lớp giáp phòng ngự', '018f000e-0000-7000-8000-0000000001f6', '018f000e-0000-7000-8000-0000000001f7', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f000e-0000-7000-8000-0000000002bf', 'Bùng nổ ham muốn ngột ngạt', '018f000e-0000-7000-8000-0000000001f7', '018f000e-0000-7000-8000-0000000001f8', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f000e-0000-7000-8000-0000000002c0', 'Sụp đổ vì nghi án phản bội', '018f000e-0000-7000-8000-0000000001f8', '018f000e-0000-7000-8000-0000000001f9', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f000e-0000-7000-8000-0000000002c1', 'Minh oan và thề nguyện trọn kiếp', '018f000e-0000-7000-8000-0000000001f9', '018f000e-0000-7000-8000-0000000001fa', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

-- 9. Table constraints & motif_constraints
INSERT INTO constraints (id, code, require, reason) VALUES
  ('018f000e-0000-7000-8000-000000000321', 'e2l_real_initial_conflict', 'Xung đột ban đầu giữa hai nhân vật phải là xung đột thực tế có căn nguyên chính đáng (lập trường, gia tộc), không được là hiểu lầm hời hợt ngớ ngẩn.', 'Sức hấp dẫn của mô-típ phụ thuộc vào độ cao của bức tường ngăn cách ban đầu.')
  ON CONFLICT (code) DO UPDATE SET require = EXCLUDED.require, reason = EXCLUDED.reason;

INSERT INTO motif_constraints (id, code, constraint_id, motif_id, syntax, constrain) VALUES
  ('018f000e-0000-7000-8000-000000000385', 'enemies_to_lovers_progression_e2l_real_initial_conflict', (SELECT id FROM constraints WHERE code = 'e2l_real_initial_conflict'), '018f000e-0000-7000-8000-000000000001', 'VALIDATE_MOTIF_RULE(''enemies_to_lovers_progression'', ''e2l_real_initial_conflict'')', 'Xung đột ban đầu giữa hai nhân vật phải là xung đột thực tế có căn nguyên chính đáng (lập trường, gia tộc), không được là hiểu lầm hời hợt ngớ ngẩn.')
  ON CONFLICT (code) DO UPDATE SET syntax = EXCLUDED.syntax, constrain = EXCLUDED.constrain;

INSERT INTO constraints (id, code, require, reason) VALUES
  ('018f000e-0000-7000-8000-000000000322', 'e2l_no_unearned_kiss', 'Nụ hôn đầu tiên không được diễn ra trước khi hai bên đã chứng kiến ít nhất một khía cạnh yếu mềm hoặc danh dự của đối phương.', 'Ngăn chặn tình yêu bị chê là nông nổi, hời hợt.')
  ON CONFLICT (code) DO UPDATE SET require = EXCLUDED.require, reason = EXCLUDED.reason;

INSERT INTO motif_constraints (id, code, constraint_id, motif_id, syntax, constrain) VALUES
  ('018f000e-0000-7000-8000-000000000386', 'enemies_to_lovers_progression_e2l_no_unearned_kiss', (SELECT id FROM constraints WHERE code = 'e2l_no_unearned_kiss'), '018f000e-0000-7000-8000-000000000001', 'VALIDATE_MOTIF_RULE(''enemies_to_lovers_progression'', ''e2l_no_unearned_kiss'')', 'Nụ hôn đầu tiên không được diễn ra trước khi hai bên đã chứng kiến ít nhất một khía cạnh yếu mềm hoặc danh dự của đối phương.')
  ON CONFLICT (code) DO UPDATE SET syntax = EXCLUDED.syntax, constrain = EXCLUDED.constrain;

-- Tham chiếu nghiên cứu thị trường: + [TV Tropes - Mentor Archetype & Master-Apprentice Bond](https://tvtropes.org/pmwiki/pmwiki.php/Main/MentorArchetype):
-- 1. Table motifs
INSERT INTO motifs (id, code, display_name, category, version, is_system, description, min_chapter, max_chapter) VALUES
  ('018f000f-0000-7000-8000-000000000001', 'mentor_protege_dynamic', 'Mối quan hệ Tôn sư & Đồ đệ (Mentor-Protégé Dynamic)', 'relationship', '1.0.0', true, 'Mô-típ gắn kết giữa bậc tiền bối thông tuệ và đệ tử kế nghiệp: Từ giai đoạn nhận đệ tử ngỗ ngược, khổ luyện truyền thừa, xung đột tư tưởng về giới hạn quyền năng, cho đến bi kịch hy sinh của người thầy và sự vượt ngưỡng của người học trò.', 25, 150);

-- 2. Table motif_applicable_genres
INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f000f-0000-7000-8000-000000000015', '018f000f-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'wuxia'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f000f-0000-7000-8000-000000000016', '018f000f-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'fantasy'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f000f-0000-7000-8000-000000000017', '018f000f-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'shounen'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f000f-0000-7000-8000-000000001017', '018f000f-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'sports'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

-- 3. Table state_rules
INSERT INTO state_rules (id, code, display_name) VALUES
  ('018f000f-0000-7000-8000-000000000029', 'torch_passing_irreversibility', 'Quy tắc chuyển giao ngọn đuốc truyền thừa một đi không trở lại')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO state_rules (id, code, display_name) VALUES
  ('018f000f-0000-7000-8000-00000000002a', 'surpass_the_teacher_mandate', 'Quy tắc đệ tử bắt buộc phải vượt qua cảnh giới của sư phụ')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

-- 4. Table states & state_down_by_rules
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f000f-0000-7000-8000-000000000065', 'mp_unrefined_scouting', '1. Thu nhận Ngọc thô Ngỗ ngược (Rough Diamond Scouting)', 'Sư phụ nhìn thấu tiềm năng ẩn giấu sau vẻ ngoài bất kham, nhận đồ đệ bất chấp tông môn phản đối.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f000f-0000-7000-8000-0000000000d2', '018f000f-0000-7000-8000-000000000065', (SELECT id FROM state_rules WHERE code = 'torch_passing_irreversibility'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f000f-0000-7000-8000-000000000066', 'mp_grueling_orthodoxy_trials', '2. Khổ luyện Truyền thừa Tâm pháp (Grueling Esoteric Training)', 'Gánh nước, bổ củi, tôi rèn căn cơ; sư phụ vừa nghiêm khắc vừa âm thầm bảo bọc đồ đệ.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f000f-0000-7000-8000-0000000000dc', '018f000f-0000-7000-8000-000000000066', (SELECT id FROM state_rules WHERE code = 'torch_passing_irreversibility'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f000f-0000-7000-8000-000000000067', 'mp_philosophical_divergence', '3. Xung đột Quan điểm Võ đạo (Philosophical Rift)', 'Học trò khao khát tốc độ và sức mạnh thực dụng, chất vấn phương pháp bảo thủ của sư phụ.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f000f-0000-7000-8000-0000000000e7', '018f000f-0000-7000-8000-000000000067', (SELECT id FROM state_rules WHERE code = 'surpass_the_teacher_mandate'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f000f-0000-7000-8000-000000000068', 'mp_mentor_sacrificial_shield', '4. Sư phụ Chắn đòn Hy sinh (Mentor Occupational Hazard)', 'Kẻ thù cũ tập kích; sư phụ dốc cạn tu vi thiêu đốt sinh mệnh để chặn hậu mở đường máu cho đệ tử trốn thoát.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f000f-0000-7000-8000-0000000000f0', '018f000f-0000-7000-8000-000000000068', (SELECT id FROM state_rules WHERE code = 'torch_passing_irreversibility'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f000f-0000-7000-8000-000000000069', 'mp_surpassing_the_master', '5. Đồ đệ Vượt ngưỡng Báo ân (Surpassing the Master & Honoring Legacy)', 'Đồ đệ giác ngộ chân lý của thầy, hoàn thiện chiêu thức cuối cùng, tiêu diệt đại địch và lập nên môn phái mới.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f000f-0000-7000-8000-0000000000fb', '018f000f-0000-7000-8000-000000000069', (SELECT id FROM state_rules WHERE code = 'surpass_the_teacher_mandate'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

-- 5. Table components
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f000f-0000-7000-8000-00000000012d', 'cmp_bamboo_stick_reprimand', 'Gậy Trúc Răn đe Rèn chí', 'Cú gõ đầu thức tỉnh tâm trí lơ đãng của thiếu niên kiêu ngạo.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO components (id, code, display_name, description) VALUES
  ('018f000f-0000-7000-8000-00000000012e', 'cmp_secret_manual_handover', 'Trao gửi Tàn quyển Trấn phái', 'Chiếc hộp gỗ chứa bí tịch thấm đẫm mồ hôi bao đời tôn sư.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO components (id, code, display_name, description) VALUES
  ('018f000f-0000-7000-8000-00000000012f', 'cmp_stormy_rooftop_argument', 'Tranh cãi Dưới Mưa trên Mái đình', 'Đệ tử quay lưng bỏ đi vì không đồng tình với sự nhẫn nhục.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO components (id, code, display_name, description) VALUES
  ('018f000f-0000-7000-8000-000000000130', 'cmp_shattering_talisman_last_stand', 'Bóp nát Linh phù Đoạn hậu', 'Ánh mắt thanh thản của sư phụ khi đóng sập cửa hầm ngầm.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO components (id, code, display_name, description) VALUES
  ('018f000f-0000-7000-8000-000000000131', 'cmp_carving_tablet_succession', 'Khắc bia Tưởng niệm Kế nghiệp', 'Dâng chén rượu lên mộ sư phụ rồi rút kiếm xuất sư.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

-- 6. Table emotional_ranges
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f000f-0000-7000-8000-000000000191', 'emo_rebellious_bravado', 'Ngỗ ngược Bất phục')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f000f-0000-7000-8000-000000000192', 'emo_exhausted_gratitude', 'Kiệt sức Biết ơn')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f000f-0000-7000-8000-000000000193', 'emo_frustrated_indignation', 'Ức chế Bất bình')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f000f-0000-7000-8000-000000000194', 'emo_gut_wrenching_anguish', 'Đứt ruột Đau đớn')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f000f-0000-7000-8000-000000000195', 'emo_solemn_transcendence', 'Trang nghiêm Siêu việt')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

-- 7. Table event_blocks & event_block_emotionals
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f000f-0000-7000-8000-0000000001f5', 'eb_mp_scout', '018f000f-0000-7000-8000-00000000012d', '018f000f-0000-7000-8000-000000000065', '018f000f-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f000f-0000-7000-8000-000000000262', (SELECT id FROM emotional_ranges WHERE code = 'emo_rebellious_bravado'), '018f000f-0000-7000-8000-0000000001f5')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f000f-0000-7000-8000-0000000001f6', 'eb_mp_train', '018f000f-0000-7000-8000-00000000012e', '018f000f-0000-7000-8000-000000000066', '018f000f-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f000f-0000-7000-8000-00000000026d', (SELECT id FROM emotional_ranges WHERE code = 'emo_exhausted_gratitude'), '018f000f-0000-7000-8000-0000000001f6')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f000f-0000-7000-8000-0000000001f7', 'eb_mp_rift', '018f000f-0000-7000-8000-00000000012f', '018f000f-0000-7000-8000-000000000067', '018f000f-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f000f-0000-7000-8000-000000000278', (SELECT id FROM emotional_ranges WHERE code = 'emo_frustrated_indignation'), '018f000f-0000-7000-8000-0000000001f7')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f000f-0000-7000-8000-0000000001f8', 'eb_mp_sacrifice', '018f000f-0000-7000-8000-000000000130', '018f000f-0000-7000-8000-000000000068', '018f000f-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f000f-0000-7000-8000-000000000283', (SELECT id FROM emotional_ranges WHERE code = 'emo_gut_wrenching_anguish'), '018f000f-0000-7000-8000-0000000001f8')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f000f-0000-7000-8000-0000000001f9', 'eb_mp_surpass', '018f000f-0000-7000-8000-000000000131', '018f000f-0000-7000-8000-000000000069', '018f000f-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f000f-0000-7000-8000-00000000028e', (SELECT id FROM emotional_ranges WHERE code = 'emo_solemn_transcendence'), '018f000f-0000-7000-8000-0000000001f9')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

-- 8. Table event_transitions
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f000f-0000-7000-8000-0000000002bd', 'Bước vào sơn môn khổ tu', '018f000f-0000-7000-8000-0000000001f5', '018f000f-0000-7000-8000-0000000001f6', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f000f-0000-7000-8000-0000000002be', 'Rạn nứt vì bất đồng quan điểm', '018f000f-0000-7000-8000-0000000001f6', '018f000f-0000-7000-8000-0000000001f7', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f000f-0000-7000-8000-0000000002bf', 'Bi kịch giáng xuống đền đáp công ơn', '018f000f-0000-7000-8000-0000000001f7', '018f000f-0000-7000-8000-0000000001f8', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f000f-0000-7000-8000-0000000002c0', 'Hóa bi thương thành sức mạnh cái thế', '018f000f-0000-7000-8000-0000000001f8', '018f000f-0000-7000-8000-0000000001f9', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

-- 9. Table constraints & motif_constraints
INSERT INTO constraints (id, code, require, reason) VALUES
  ('018f000f-0000-7000-8000-000000000321', 'mp_mentor_must_pass_torch', 'Sư phụ phải trao lại biểu tượng uy quyền hoặc tri thức cốt lõi trước khi rút khỏi vị thế bảo hộ.', 'Tạo tính hợp pháp kế vị của người đệ tử.')
  ON CONFLICT (code) DO UPDATE SET require = EXCLUDED.require, reason = EXCLUDED.reason;

INSERT INTO motif_constraints (id, code, constraint_id, motif_id, syntax, constrain) VALUES
  ('018f000f-0000-7000-8000-000000000385', 'mentor_protege_dynamic_mp_mentor_must_pass_torch', (SELECT id FROM constraints WHERE code = 'mp_mentor_must_pass_torch'), '018f000f-0000-7000-8000-000000000001', 'VALIDATE_MOTIF_RULE(''mentor_protege_dynamic'', ''mp_mentor_must_pass_torch'')', 'Sư phụ phải trao lại biểu tượng uy quyền hoặc tri thức cốt lõi trước khi rút khỏi vị thế bảo hộ.')
  ON CONFLICT (code) DO UPDATE SET syntax = EXCLUDED.syntax, constrain = EXCLUDED.constrain;

INSERT INTO constraints (id, code, require, reason) VALUES
  ('018f000f-0000-7000-8000-000000000322', 'mp_disciple_must_not_stay_shadow', 'Đồ đệ ở giai đoạn cuối bắt buộc phải giải quyết được vấn đề mà thế hệ sư phụ từng bất lực bó tay.', 'Thể hiện sự tiến bộ của lịch sử và sự trưởng thành chân thực.')
  ON CONFLICT (code) DO UPDATE SET require = EXCLUDED.require, reason = EXCLUDED.reason;

INSERT INTO motif_constraints (id, code, constraint_id, motif_id, syntax, constrain) VALUES
  ('018f000f-0000-7000-8000-000000000386', 'mentor_protege_dynamic_mp_disciple_must_not_stay_shadow', (SELECT id FROM constraints WHERE code = 'mp_disciple_must_not_stay_shadow'), '018f000f-0000-7000-8000-000000000001', 'VALIDATE_MOTIF_RULE(''mentor_protege_dynamic'', ''mp_disciple_must_not_stay_shadow'')', 'Đồ đệ ở giai đoạn cuối bắt buộc phải giải quyết được vấn đề mà thế hệ sư phụ từng bất lực bó tay.')
  ON CONFLICT (code) DO UPDATE SET syntax = EXCLUDED.syntax, constrain = EXCLUDED.constrain;

-- Tham chiếu nghiên cứu thị trường: + [TV Tropes - Defeat Means Friendship & Rivals Team Up Dynamic](https://tvtropes.org/pmwiki/pmwiki.php/Main/DefeatMeansFriendship):
-- 1. Table motifs
INSERT INTO motifs (id, code, display_name, category, version, is_system, description, min_chapter, max_chapter) VALUES
  ('018f0010-0000-7000-8000-000000000001', 'rivals_to_staunch_allies', 'Kình địch hóa Đồng minh Sinh tử (Rivals to Allies Dynamic)', 'relationship', '1.0.0', true, 'Mô-típ kình phùng địch thủ: Hai thiên tài có tính cách trái ngược (Lửa và Băng, Ánh sáng và Bóng tối), cạnh tranh từng bước chân. Sau trận huyết chiến định đoạt hoặc trước sự xuất hiện của mối đe dọa hủy diệt chung, họ bắt tay kề vai sát cánh thành cặp bài trùng bất khả chiến bại.', 20, 120);

-- 2. Table motif_applicable_genres
INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f0010-0000-7000-8000-000000000015', '018f0010-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'shounen'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f0010-0000-7000-8000-000000000016', '018f0010-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'xianxia'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f0010-0000-7000-8000-000000001016', '018f0010-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'urban_dushi'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f0010-0000-7000-8000-000000000017', '018f0010-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'mecha'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

-- 3. Table state_rules
INSERT INTO state_rules (id, code, display_name) VALUES
  ('018f0010-0000-7000-8000-000000000029', 'mirror_foil_personality_rule', 'Quy tắc tính cách tương phản bổ trợ (Foil Dynamic)')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO state_rules (id, code, display_name) VALUES
  ('018f0010-0000-7000-8000-00000000002a', 'mutual_respect_through_combat', 'Quy tắc tôn trọng được xây dựng qua đòn thế sinh tử')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

-- 4. Table states & state_down_by_rules
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0010-0000-7000-8000-000000000065', 'ra_fiery_competitiveness', '1. Cạnh tranh Gay gắt Bất phân Thắng bại (Fierce Ideological Rivalry)', 'Luôn so kè từng điểm số, từng đòn thế; coi đối phương là rào cản lớn nhất phải vượt qua.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0010-0000-7000-8000-0000000000d2', '018f0010-0000-7000-8000-000000000065', (SELECT id FROM state_rules WHERE code = 'mirror_foil_personality_rule'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0010-0000-7000-8000-000000000066', 'ra_climactic_duel_of_honor', '2. Huyết chiến Quyết định Danh dự (Climactic Duel of Honor)', 'Trận đấu riêng tư không khoan nhượng; cả hai bộc lộ hết lá bài tẩy, nhận ra tri kỷ duy nhất thấu hiểu mình.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0010-0000-7000-8000-0000000000dd', '018f0010-0000-7000-8000-000000000066', (SELECT id FROM state_rules WHERE code = 'mutual_respect_through_combat'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0010-0000-7000-8000-000000000067', 'ra_cataclysmic_common_foe', '3. Hiểm họa Tối thượng Xuất hiện (Greater Existential Threat)', 'Thế lực hắc ám hủy diệt ập đến đe dọa sinh tồn của cả hai; buộc phải tạm gác tranh chấp cá nhân.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0010-0000-7000-8000-0000000000e7', '018f0010-0000-7000-8000-000000000067', (SELECT id FROM state_rules WHERE code = 'mutual_respect_through_combat'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0010-0000-7000-8000-000000000068', 'ra_flawless_combo_symphony', '4. Phối hợp Tác chiến Lưng tựa Lưng (Back-to-Back Synergy)', 'Không cần lời nói, một ánh mắt là hiểu ý; chiêu thức hai bên bù trừ hoàn hảo hạ gục quái vật.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0010-0000-7000-8000-0000000000f0', '018f0010-0000-7000-8000-000000000068', (SELECT id FROM state_rules WHERE code = 'mirror_foil_personality_rule'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0010-0000-7000-8000-000000000069', 'ra_eternal_unspoken_bond', '5. Huynh đệ Tri kỷ Trọn đời (Unspoken Lifelong Brotherhood)', 'Tiếp tục cà khịa ngoài miệng nhưng sẵn sàng đỡ kiếm cho nhau trong chớp mắt.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0010-0000-7000-8000-0000000000fa', '018f0010-0000-7000-8000-000000000069', (SELECT id FROM state_rules WHERE code = 'mirror_foil_personality_rule'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

-- 5. Table components
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0010-0000-7000-8000-00000000012d', 'cmp_smug_scoreboard_taunt', 'Khiêu khích Bảng điểm Chiến tích', '''Hôm nay ta hạ 50 tên, ngươi mới có 49 thôi đấy!''.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0010-0000-7000-8000-00000000012e', 'cmp_cross_counter_simultaneous_fist', 'Đấm Trúng Mặt Nhau Cùng Lúc', 'Cả hai ngã nhào ra đất rồi cùng bật cười sảng khoái.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0010-0000-7000-8000-00000000012f', 'cmp_world_devouring_sky_crack', 'Bầu trời Nứt vỡ Báo hiệu Tận thế', 'Quân đoàn hắc ám kéo đến san bằng võ đài thi đấu.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0010-0000-7000-8000-000000000130', 'cmp_interlocking_spear_and_shield', 'Thương Khiên Giao thoa Bọc lót', 'Một người tạo rào chắn phòng thủ tuyệt đối, người kia tung đòn kết liễu chí mạng.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0010-0000-7000-8000-000000000131', 'cmp_fist_bump_silent_nod', 'Chạm Nắm đấm & Cái gật đầu Im lặng', 'Hành động ngắn gọn khẳng định sự tin tưởng giao phó tính mạng.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

-- 6. Table emotional_ranges
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0010-0000-7000-8000-000000000191', 'emo_fiery_jealousy', 'Ganh đua Bừng bừng')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0010-0000-7000-8000-000000000192', 'emo_exhilarating_resonance', 'Đồng điệu Thăng hoa')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0010-0000-7000-8000-000000000193', 'emo_grim_sobering_crisis', 'Nghiêm trọng Nguy ngập')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0010-0000-7000-8000-000000000194', 'emo_telepathic_euphoria', 'Ăn ý Tuyệt đối')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0010-0000-7000-8000-000000000195', 'emo_fierce_camaraderie', 'Tình bạn Thép Kiên cố')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

-- 7. Table event_blocks & event_block_emotionals
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0010-0000-7000-8000-0000000001f5', 'eb_ra_rival', '018f0010-0000-7000-8000-00000000012d', '018f0010-0000-7000-8000-000000000065', '018f0010-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0010-0000-7000-8000-000000000262', (SELECT id FROM emotional_ranges WHERE code = 'emo_fiery_jealousy'), '018f0010-0000-7000-8000-0000000001f5')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0010-0000-7000-8000-0000000001f6', 'eb_ra_duel', '018f0010-0000-7000-8000-00000000012e', '018f0010-0000-7000-8000-000000000066', '018f0010-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0010-0000-7000-8000-00000000026d', (SELECT id FROM emotional_ranges WHERE code = 'emo_exhilarating_resonance'), '018f0010-0000-7000-8000-0000000001f6')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0010-0000-7000-8000-0000000001f7', 'eb_ra_foe', '018f0010-0000-7000-8000-00000000012f', '018f0010-0000-7000-8000-000000000067', '018f0010-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0010-0000-7000-8000-000000000278', (SELECT id FROM emotional_ranges WHERE code = 'emo_grim_sobering_crisis'), '018f0010-0000-7000-8000-0000000001f7')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0010-0000-7000-8000-0000000001f8', 'eb_ra_combo', '018f0010-0000-7000-8000-000000000130', '018f0010-0000-7000-8000-000000000068', '018f0010-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0010-0000-7000-8000-000000000283', (SELECT id FROM emotional_ranges WHERE code = 'emo_telepathic_euphoria'), '018f0010-0000-7000-8000-0000000001f8')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0010-0000-7000-8000-0000000001f9', 'eb_ra_bond', '018f0010-0000-7000-8000-000000000131', '018f0010-0000-7000-8000-000000000069', '018f0010-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0010-0000-7000-8000-00000000028e', (SELECT id FROM emotional_ranges WHERE code = 'emo_fierce_camaraderie'), '018f0010-0000-7000-8000-0000000001f9')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

-- 8. Table event_transitions
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0010-0000-7000-8000-0000000002bd', 'Trận phân định đỉnh cao', '018f0010-0000-7000-8000-0000000001f5', '018f0010-0000-7000-8000-0000000001f6', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0010-0000-7000-8000-0000000002be', 'Bão táp chung ập xuống', '018f0010-0000-7000-8000-0000000001f6', '018f0010-0000-7000-8000-0000000001f7', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0010-0000-7000-8000-0000000002bf', 'Lưng tựa lưng nghênh địch', '018f0010-0000-7000-8000-0000000001f7', '018f0010-0000-7000-8000-0000000001f8', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0010-0000-7000-8000-0000000002c0', 'Định hình tình tri kỷ bất tử', '018f0010-0000-7000-8000-0000000001f8', '018f0010-0000-7000-8000-0000000001f9', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

-- 9. Table constraints & motif_constraints
INSERT INTO constraints (id, code, require, reason) VALUES
  ('018f0010-0000-7000-8000-000000000321', 'ra_foil_complementarity', 'Hai nhân vật phải có ít nhất 2 đặc điểm tương phản trực tiếp (tính cách bốc đồng vs trầm tĩnh, cận chiến vs xạ thủ).', 'Tạo ra sự bổ khuyết hoàn hảo khi kết hợp lực lượng.')
  ON CONFLICT (code) DO UPDATE SET require = EXCLUDED.require, reason = EXCLUDED.reason;

INSERT INTO motif_constraints (id, code, constraint_id, motif_id, syntax, constrain) VALUES
  ('018f0010-0000-7000-8000-000000000385', 'rivals_to_staunch_allies_ra_foil_complementarity', (SELECT id FROM constraints WHERE code = 'ra_foil_complementarity'), '018f0010-0000-7000-8000-000000000001', 'VALIDATE_MOTIF_RULE(''rivals_to_staunch_allies'', ''ra_foil_complementarity'')', 'Hai nhân vật phải có ít nhất 2 đặc điểm tương phản trực tiếp (tính cách bốc đồng vs trầm tĩnh, cận chiến vs xạ thủ).')
  ON CONFLICT (code) DO UPDATE SET syntax = EXCLUDED.syntax, constrain = EXCLUDED.constrain;

INSERT INTO constraints (id, code, require, reason) VALUES
  ('018f0010-0000-7000-8000-000000000322', 'ra_banter_retention', 'Dù đã trở thành đồng minh sinh tử, cả hai vẫn phải giữ thói quen mỉa mai trêu chọc lẫn nhau khi tình thế lắng dịu.', 'Giữ đúng nét đặc sắc đáng yêu của trope Rivals to Allies.')
  ON CONFLICT (code) DO UPDATE SET require = EXCLUDED.require, reason = EXCLUDED.reason;

INSERT INTO motif_constraints (id, code, constraint_id, motif_id, syntax, constrain) VALUES
  ('018f0010-0000-7000-8000-000000000386', 'rivals_to_staunch_allies_ra_banter_retention', (SELECT id FROM constraints WHERE code = 'ra_banter_retention'), '018f0010-0000-7000-8000-000000000001', 'VALIDATE_MOTIF_RULE(''rivals_to_staunch_allies'', ''ra_banter_retention'')', 'Dù đã trở thành đồng minh sinh tử, cả hai vẫn phải giữ thói quen mỉa mai trêu chọc lẫn nhau khi tình thế lắng dịu.')
  ON CONFLICT (code) DO UPDATE SET syntax = EXCLUDED.syntax, constrain = EXCLUDED.constrain;

-- Tham chiếu nghiên cứu thị trường: + [Literary Devices - Hollywood Symbolism: The Mirror and The Mask](https://literarydevices.net/hollywood-symbolism/):
-- 1. Table motifs
INSERT INTO motifs (id, code, display_name, category, version, is_system, description, min_chapter, max_chapter) VALUES
  ('018f0011-0000-7000-8000-000000000001', 'the_mirror_and_mask_symbol', 'Biểu tượng Tấm gương & Mặt nạ (The Mirror & The Mask)', 'symbolic', '1.0.0', true, 'Mô-típ biểu tượng chiều sâu văn học: Mặt nạ đại diện cho bản ngã xã hội (Persona), sự che giấu tội lỗi hoặc lớp vỏ tự vệ; Tấm gương đại diện cho sự tự soi chiếu, đối diện nhân cách bóng tối (Shadow Self) và sự vỡ vụn của ảo tưởng.', 15, 80);

-- 2. Table motif_applicable_genres
INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f0011-0000-7000-8000-000000000015', '018f0011-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'mystery_detective'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f0011-0000-7000-8000-000000001015', '018f0011-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'psychological'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f0011-0000-7000-8000-000000000016', '018f0011-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'horror'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f0011-0000-7000-8000-000000000017', '018f0011-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'dark'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

-- 3. Table state_rules
INSERT INTO state_rules (id, code, display_name) VALUES
  ('018f0011-0000-7000-8000-000000000029', 'symbolic_fracture_reflection', 'Quy tắc vết rạn trên gương phản chiếu sự đổ vỡ tâm thần')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO state_rules (id, code, display_name) VALUES
  ('018f0011-0000-7000-8000-00000000002a', 'mask_peeling_exposure', 'Quy tắc lột mặt nạ phơi bày bản thể trần trụi')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

-- 4. Table states & state_down_by_rules
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0011-0000-7000-8000-000000000065', 'mm_impenetrable_persona', '1. Mặt nạ Hoàn hảo & Gương Phẳng lặng (Pristine Reflection)', 'Nhân vật đeo mặt nạ đạo mạo, tự mãn với hình tượng xã hội hoàn mỹ trong gương.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0011-0000-7000-8000-0000000000d2', '018f0011-0000-7000-8000-000000000065', (SELECT id FROM state_rules WHERE code = 'symbolic_fracture_reflection'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0011-0000-7000-8000-000000000066', 'mm_micro_cracks_whisper', '2. Vết rạn Li ti & Tiếng thì thầm (Micro-Fissures in the Glass)', 'Một biến cố nhỏ làm xuất hiện vết nứt trên kính; hình ảnh phản chiếu bắt đầu cười khi nhân vật khóc.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0011-0000-7000-8000-0000000000dc', '018f0011-0000-7000-8000-000000000066', (SELECT id FROM state_rules WHERE code = 'symbolic_fracture_reflection'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0011-0000-7000-8000-000000000067', 'mm_nightmarish_distortion', '3. Khúc xạ Biến dạng Nhân cách (Distorted Shadow Confrontation)', 'Đối diện với chiếc gương méo mó; con quái vật bên trong cào cấu đòi thoát ra ngoài.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0011-0000-7000-8000-0000000000e6', '018f0011-0000-7000-8000-000000000067', (SELECT id FROM state_rules WHERE code = 'symbolic_fracture_reflection'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0011-0000-7000-8000-000000000068', 'mm_shattering_the_glass', '4. Gương Vỡ tan & Mặt nạ Rơi rụng (The Shattered Mask & Glass)', 'Khoảnh khắc đập tan tấm gương hoặc xé toạc mặt nạ; ảo tưởng che đậy chính thức tan biến.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0011-0000-7000-8000-0000000000f1', '018f0011-0000-7000-8000-000000000068', (SELECT id FROM state_rules WHERE code = 'mask_peeling_exposure'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0011-0000-7000-8000-000000000069', 'mm_naked_authenticity_rebirth', '5. Bản ngã Chân thực Tái sinh (Naked Self Integration)', 'Nhìn vào hàng ngàn mảnh vỡ nhỏ, chấp nhận sự không hoàn hảo và hòa hợp với phần bóng tối.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0011-0000-7000-8000-0000000000fb', '018f0011-0000-7000-8000-000000000069', (SELECT id FROM state_rules WHERE code = 'mask_peeling_exposure'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

-- 5. Table components
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0011-0000-7000-8000-00000000012d', 'cmp_silver_gilded_frame', 'Khung Gương Bạc Mạ vàng', 'Chăm chút trang phục và nụ cười công nghiệp trước buổi dạ vũ.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0011-0000-7000-8000-00000000012e', 'cmp_spiderweb_crack_appearance', 'Màng Nhện Nứt nẻ Đột ngột', 'Một viên sỏi va vào kính làm lộ đường nứt chạy dọc con ngươi.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0011-0000-7000-8000-00000000012f', 'cmp_monstrous_reflection_reach', 'Cánh tay Bóng ma Vươn khỏi Gương', 'Bản sao tăm tối tóm lấy cổ tay nhân vật cảnh báo sự suy đồi.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0011-0000-7000-8000-000000000130', 'cmp_bloody_knuckle_glass_strike', 'Cú đấm Nát Gương Nhuốm máu', 'Vung nắm đấm phá nát hình ảnh giả dối trong cơn phẫn nộ cùng cực.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0011-0000-7000-8000-000000000131', 'cmp_holding_tiny_shard_smile', 'Cầm Mảnh Vỡ Nhỏ Mỉm cười', 'Nhìn thấy đôi mắt mộc mạc không còn lớp trang điểm che giấu.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

-- 6. Table emotional_ranges
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0011-0000-7000-8000-000000000191', 'emo_varnished_complacency', 'Thỏa mãn Giả tạo')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0011-0000-7000-8000-000000000192', 'emo_creeping_dread_itch', 'Bất an Bứt rứt')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0011-0000-7000-8000-000000000193', 'emo_hallucinatory_vertigo', 'Chóng mặt Hoang tưởng')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0011-0000-7000-8000-000000000194', 'emo_violent_shattering_relief', 'Đổ vỡ Giải thoát Khốc liệt')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0011-0000-7000-8000-000000000195', 'emo_quiet_wholeness', 'Toàn vẹn Lặng lẽ')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

-- 7. Table event_blocks & event_block_emotionals
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0011-0000-7000-8000-0000000001f5', 'eb_mm_pristine', '018f0011-0000-7000-8000-00000000012d', '018f0011-0000-7000-8000-000000000065', '018f0011-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0011-0000-7000-8000-000000000262', (SELECT id FROM emotional_ranges WHERE code = 'emo_varnished_complacency'), '018f0011-0000-7000-8000-0000000001f5')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0011-0000-7000-8000-0000000001f6', 'eb_mm_cracks', '018f0011-0000-7000-8000-00000000012e', '018f0011-0000-7000-8000-000000000066', '018f0011-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0011-0000-7000-8000-00000000026d', (SELECT id FROM emotional_ranges WHERE code = 'emo_creeping_dread_itch'), '018f0011-0000-7000-8000-0000000001f6')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0011-0000-7000-8000-0000000001f7', 'eb_mm_shadow', '018f0011-0000-7000-8000-00000000012f', '018f0011-0000-7000-8000-000000000067', '018f0011-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0011-0000-7000-8000-000000000278', (SELECT id FROM emotional_ranges WHERE code = 'emo_hallucinatory_vertigo'), '018f0011-0000-7000-8000-0000000001f7')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0011-0000-7000-8000-0000000001f8', 'eb_mm_shatter', '018f0011-0000-7000-8000-000000000130', '018f0011-0000-7000-8000-000000000068', '018f0011-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0011-0000-7000-8000-000000000283', (SELECT id FROM emotional_ranges WHERE code = 'emo_violent_shattering_relief'), '018f0011-0000-7000-8000-0000000001f8')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0011-0000-7000-8000-0000000001f9', 'eb_mm_naked', '018f0011-0000-7000-8000-000000000131', '018f0011-0000-7000-8000-000000000069', '018f0011-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0011-0000-7000-8000-00000000028e', (SELECT id FROM emotional_ranges WHERE code = 'emo_quiet_wholeness'), '018f0011-0000-7000-8000-0000000001f9')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

-- 8. Table event_transitions
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0011-0000-7000-8000-0000000002bd', 'Vết rạn đầu tiên xuất hiện', '018f0011-0000-7000-8000-0000000001f5', '018f0011-0000-7000-8000-0000000001f6', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0011-0000-7000-8000-0000000002be', 'Bóng tối đòi quyền kiểm soát', '018f0011-0000-7000-8000-0000000001f6', '018f0011-0000-7000-8000-0000000001f7', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0011-0000-7000-8000-0000000002bf', 'Đập nát ảo ảnh ngụy tạo', '018f0011-0000-7000-8000-0000000001f7', '018f0011-0000-7000-8000-0000000001f8', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0011-0000-7000-8000-0000000002c0', 'Tái sinh từ mảnh vụn chân thật', '018f0011-0000-7000-8000-0000000001f8', '018f0011-0000-7000-8000-0000000001f9', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

-- 9. Table constraints & motif_constraints
INSERT INTO constraints (id, code, require, reason) VALUES
  ('018f0011-0000-7000-8000-000000000321', 'mm_mirror_repetition_rule', 'Hình tượng tấm gương hoặc mặt nạ phải xuất hiện ít nhất 3 lần tại các bước ngoặt cảm xúc trọng đại.', 'Đảm bảo tính nhất quán của motif biểu tượng xuyên suốt tác phẩm.')
  ON CONFLICT (code) DO UPDATE SET require = EXCLUDED.require, reason = EXCLUDED.reason;

INSERT INTO motif_constraints (id, code, constraint_id, motif_id, syntax, constrain) VALUES
  ('018f0011-0000-7000-8000-000000000385', 'the_mirror_and_mask_symbol_mm_mirror_repetition_rule', (SELECT id FROM constraints WHERE code = 'mm_mirror_repetition_rule'), '018f0011-0000-7000-8000-000000000001', 'VALIDATE_MOTIF_RULE(''the_mirror_and_mask_symbol'', ''mm_mirror_repetition_rule'')', 'Hình tượng tấm gương hoặc mặt nạ phải xuất hiện ít nhất 3 lần tại các bước ngoặt cảm xúc trọng đại.')
  ON CONFLICT (code) DO UPDATE SET syntax = EXCLUDED.syntax, constrain = EXCLUDED.constrain;

INSERT INTO constraints (id, code, require, reason) VALUES
  ('018f0011-0000-7000-8000-000000000322', 'mm_physical_mirror_shattering', 'Hành vi vỡ gương phải đi kèm với chấn thương thể xác hoặc sự trần trụi hóa thân phận xã hội.', 'Liên kết trực quan giữa biểu tượng vật lý và sự chuyển hóa tâm thức.')
  ON CONFLICT (code) DO UPDATE SET require = EXCLUDED.require, reason = EXCLUDED.reason;

INSERT INTO motif_constraints (id, code, constraint_id, motif_id, syntax, constrain) VALUES
  ('018f0011-0000-7000-8000-000000000386', 'the_mirror_and_mask_symbol_mm_physical_mirror_shattering', (SELECT id FROM constraints WHERE code = 'mm_physical_mirror_shattering'), '018f0011-0000-7000-8000-000000000001', 'VALIDATE_MOTIF_RULE(''the_mirror_and_mask_symbol'', ''mm_physical_mirror_shattering'')', 'Hành vi vỡ gương phải đi kèm với chấn thương thể xác hoặc sự trần trụi hóa thân phận xã hội.')
  ON CONFLICT (code) DO UPDATE SET syntax = EXCLUDED.syntax, constrain = EXCLUDED.constrain;

-- Tham chiếu nghiên cứu thị trường: + [Literary Devices - Bird Symbolism: The Phoenix, Ashes and Immortal Rebirth](https://literarydevices.net/bird-symbolism/):
-- 1. Table motifs
INSERT INTO motifs (id, code, display_name, category, version, is_system, description, min_chapter, max_chapter) VALUES
  ('018f0012-0000-7000-8000-000000000001', 'phoenix_ashes_rebirth_symbol', 'Biểu tượng Phượng hoàng Tro tàn (The Phoenix & Ashes Rebirth)', 'symbolic', '1.0.0', true, 'Biểu tượng thần thoại phổ quát về chu kỳ Hủy diệt để Tái sinh: Nhân vật phải trải qua quá trình suy kiệt, tự thiêu rụi cái cũ trong ngọn lửa thống khổ tột cùng, chìm vào tro tàn câm lặng trước khi vút bay lên với hình hài bất tử kiêu hãnh.', 15, 85);

-- 2. Table motif_applicable_genres
INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f0012-0000-7000-8000-000000000015', '018f0012-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'fantasy'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f0012-0000-7000-8000-000000000016', '018f0012-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'contemporary_fiction'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f0012-0000-7000-8000-000000000017', '018f0012-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'xianxia'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

-- 3. Table state_rules
INSERT INTO state_rules (id, code, display_name) VALUES
  ('018f0012-0000-7000-8000-000000000029', 'total_combustion_purity', 'Quy tắc thiêu rụi hoàn toàn không còn tạp chất cũ')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO state_rules (id, code, display_name) VALUES
  ('018f0012-0000-7000-8000-00000000002a', 'ashes_incubation_period', 'Quy tắc ủ mầm trong tro tàn tịch mịch')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

-- 4. Table states & state_down_by_rules
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0012-0000-7000-8000-000000000065', 'phx_waning_plumage', '1. Lông vũ Phai màu & Sức tàn Lực kiệt (Waning Plumage)', 'Sinh lực cạn kiệt, vinh quang lụi tàn; nhân vật nhận thức rõ cái cũ đã đi đến hồi kết.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0012-0000-7000-8000-0000000000d2', '018f0012-0000-7000-8000-000000000065', (SELECT id FROM state_rules WHERE code = 'total_combustion_purity'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0012-0000-7000-8000-000000000066', 'phx_pyre_of_self_immolation', '2. Dàn hỏa Tự thiêu & Nỗi đau Niết bàn (The Sacred Pyre Combustion)', 'Chủ động bước vào ngọn lửa đau đớn; thiêu rụi toàn bộ sai lầm, ảo tưởng và thân xác cũ.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0012-0000-7000-8000-0000000000dc', '018f0012-0000-7000-8000-000000000066', (SELECT id FROM state_rules WHERE code = 'total_combustion_purity'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0012-0000-7000-8000-000000000067', 'phx_dormant_cold_ashes', '3. Tro tàn Câm lặng Ủ mầm (Dormant Smoldering Ashes)', 'Khoảng lặng tuyệt đối; thế giới bên ngoài tưởng rằng mọi thứ đã kết thúc trong vô vọng.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0012-0000-7000-8000-0000000000e7', '018f0012-0000-7000-8000-000000000067', (SELECT id FROM state_rules WHERE code = 'ashes_incubation_period'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0012-0000-7000-8000-000000000068', 'phx_golden_spark_resurgence', '4. Đốm lửa Vàng Thức tỉnh (The Golden Spark Awakening)', 'Từ giữa đống tro nguội ngắt, một tia lửa thần thánh le lói bốc cháy, quả trứng vàng nứt vỏ.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0012-0000-7000-8000-0000000000f0', '018f0012-0000-7000-8000-000000000068', (SELECT id FROM state_rules WHERE code = 'total_combustion_purity'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0012-0000-7000-8000-000000000069', 'phx_radiant_skyward_ascendance', '5. Tung cánh Phục sinh Rực rỡ (Radiant Skyward Ascendance)', 'Đôi cánh rực lửa xé toạc màn đêm, bay vút lên bầu trời với sức mạnh vĩnh cửu vô song.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0012-0000-7000-8000-0000000000fa', '018f0012-0000-7000-8000-000000000069', (SELECT id FROM state_rules WHERE code = 'total_combustion_purity'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

-- 5. Table components
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0012-0000-7000-8000-00000000012d', 'cmp_falling_grey_feather', 'Cọng Lông vũ Xám Rụng rơi', 'Dấu hiệu suy yếu không thể cứu vãn của cơ thể già cỗi.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0012-0000-7000-8000-00000000012e', 'cmp_embracing_holy_inferno', 'Ôm trọn Lửa thiêng Thần thánh', 'Bước lên ngọn lửa tế đàn không hề run sợ.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0012-0000-7000-8000-00000000012f', 'cmp_grey_powder_silence', 'Cát bụi Xám xịt Lặng câm', 'Gió thổi qua đống tro tàn nơi từng là vị thần kiêu hùng.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0012-0000-7000-8000-000000000130', 'cmp_heartbeat_beneath_embers', 'Nhịp tim Đập Dưới Tàn than', 'Âm thanh thình thịch hồi sinh đập tan sự tĩnh lặng chết chóc.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0012-0000-7000-8000-000000000131', 'cmp_blazing_wing_cloud_spread', 'Đôi Cánh Lửa Che rợp Trời', 'Tiếng hót vang vọng chín tầng trời xua tan mọi tà khí.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

-- 6. Table emotional_ranges
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0012-0000-7000-8000-000000000191', 'emo_melancholy_fading', 'Lụi tàn U sầu')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0012-0000-7000-8000-000000000192', 'emo_searing_agony', 'Đau đớn Xé ruột Thiêu đốt')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0012-0000-7000-8000-000000000193', 'emo_abyssal_stillness', 'Tĩnh mịch Đáy vực')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0012-0000-7000-8000-000000000194', 'emo_miraculous_flutter', 'Rạo rực Kỳ diệu')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0012-0000-7000-8000-000000000195', 'emo_majestic_ecstasy', 'Hân hoan Uy nghiêm')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

-- 7. Table event_blocks & event_block_emotionals
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0012-0000-7000-8000-0000000001f5', 'eb_phx_waning', '018f0012-0000-7000-8000-00000000012d', '018f0012-0000-7000-8000-000000000065', '018f0012-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0012-0000-7000-8000-000000000262', (SELECT id FROM emotional_ranges WHERE code = 'emo_melancholy_fading'), '018f0012-0000-7000-8000-0000000001f5')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0012-0000-7000-8000-0000000001f6', 'eb_phx_pyre', '018f0012-0000-7000-8000-00000000012e', '018f0012-0000-7000-8000-000000000066', '018f0012-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0012-0000-7000-8000-00000000026d', (SELECT id FROM emotional_ranges WHERE code = 'emo_searing_agony'), '018f0012-0000-7000-8000-0000000001f6')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0012-0000-7000-8000-0000000001f7', 'eb_phx_ashes', '018f0012-0000-7000-8000-00000000012f', '018f0012-0000-7000-8000-000000000067', '018f0012-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0012-0000-7000-8000-000000000278', (SELECT id FROM emotional_ranges WHERE code = 'emo_abyssal_stillness'), '018f0012-0000-7000-8000-0000000001f7')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0012-0000-7000-8000-0000000001f8', 'eb_phx_spark', '018f0012-0000-7000-8000-000000000130', '018f0012-0000-7000-8000-000000000068', '018f0012-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0012-0000-7000-8000-000000000283', (SELECT id FROM emotional_ranges WHERE code = 'emo_miraculous_flutter'), '018f0012-0000-7000-8000-0000000001f8')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0012-0000-7000-8000-0000000001f9', 'eb_phx_ascend', '018f0012-0000-7000-8000-000000000131', '018f0012-0000-7000-8000-000000000069', '018f0012-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0012-0000-7000-8000-00000000028e', (SELECT id FROM emotional_ranges WHERE code = 'emo_majestic_ecstasy'), '018f0012-0000-7000-8000-0000000001f9')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

-- 8. Table event_transitions
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0012-0000-7000-8000-0000000002bd', 'Bước vào ngọn lửa tự thiêu', '018f0012-0000-7000-8000-0000000001f5', '018f0012-0000-7000-8000-0000000001f6', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0012-0000-7000-8000-0000000002be', 'Lắng đọng thành tro tàn lạnh giá', '018f0012-0000-7000-8000-0000000001f6', '018f0012-0000-7000-8000-0000000001f7', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0012-0000-7000-8000-0000000002bf', 'Nhen nhóm tia sáng phục sinh', '018f0012-0000-7000-8000-0000000001f7', '018f0012-0000-7000-8000-0000000001f8', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0012-0000-7000-8000-0000000002c0', 'Tung cánh bay vút trời xanh', '018f0012-0000-7000-8000-0000000001f8', '018f0012-0000-7000-8000-0000000001f9', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

-- 9. Table constraints & motif_constraints
INSERT INTO constraints (id, code, require, reason) VALUES
  ('018f0012-0000-7000-8000-000000000321', 'phx_must_touch_ashes', 'Quá trình tái sinh bắt buộc phải trải qua giai đoạn tro tàn (Dormant Ashes), không được bỏ qua bước chạm đáy tuyệt đối.', 'Tạo sự tương phản mãnh liệt và ý nghĩa triết học về sự tôi rèn qua mất mát.')
  ON CONFLICT (code) DO UPDATE SET require = EXCLUDED.require, reason = EXCLUDED.reason;

INSERT INTO motif_constraints (id, code, constraint_id, motif_id, syntax, constrain) VALUES
  ('018f0012-0000-7000-8000-000000000385', 'phoenix_ashes_rebirth_symbol_phx_must_touch_ashes', (SELECT id FROM constraints WHERE code = 'phx_must_touch_ashes'), '018f0012-0000-7000-8000-000000000001', 'VALIDATE_MOTIF_RULE(''phoenix_ashes_rebirth_symbol'', ''phx_must_touch_ashes'')', 'Quá trình tái sinh bắt buộc phải trải qua giai đoạn tro tàn (Dormant Ashes), không được bỏ qua bước chạm đáy tuyệt đối.')
  ON CONFLICT (code) DO UPDATE SET syntax = EXCLUDED.syntax, constrain = EXCLUDED.constrain;

INSERT INTO constraints (id, code, require, reason) VALUES
  ('018f0012-0000-7000-8000-000000000322', 'phx_purity_elevation', 'Sau khi tái sinh, nhân vật phải đạt được cảnh giới tinh thần hoặc sức mạnh vượt trội hơn hẳn trạng thái trước khi tự thiêu.', 'Bảo đảm ý nghĩa thăng hoa của biểu tượng Phượng hoàng.')
  ON CONFLICT (code) DO UPDATE SET require = EXCLUDED.require, reason = EXCLUDED.reason;

INSERT INTO motif_constraints (id, code, constraint_id, motif_id, syntax, constrain) VALUES
  ('018f0012-0000-7000-8000-000000000386', 'phoenix_ashes_rebirth_symbol_phx_purity_elevation', (SELECT id FROM constraints WHERE code = 'phx_purity_elevation'), '018f0012-0000-7000-8000-000000000001', 'VALIDATE_MOTIF_RULE(''phoenix_ashes_rebirth_symbol'', ''phx_purity_elevation'')', 'Sau khi tái sinh, nhân vật phải đạt được cảnh giới tinh thần hoặc sức mạnh vượt trội hơn hẳn trạng thái trước khi tự thiêu.')
  ON CONFLICT (code) DO UPDATE SET syntax = EXCLUDED.syntax, constrain = EXCLUDED.constrain;

-- Tham chiếu nghiên cứu thị trường: + [TV Tropes - Reforged Blade: Shattered Weapon Renewal](https://tvtropes.org/pmwiki/pmwiki.php/Main/ReforgedBlade):
-- 1. Table motifs
INSERT INTO motifs (id, code, display_name, category, version, is_system, description, min_chapter, max_chapter) VALUES
  ('018f0013-0000-7000-8000-000000000001', 'shattered_and_reforged_blade', 'Biểu tượng Kiếm gãy Đúc lại (The Shattered & Reforged Blade)', 'symbolic', '1.0.0', true, 'Biểu tượng hiệp sĩ và di sản dòng tộc: Thanh bảo kiếm gia truyền tượng trưng cho danh dự và lý tưởng bị gãy vụn trước ác nhân. Người thừa kế phải nhặt từng mảnh vỡ, đem tôi luyện trong lò rèn định mệnh bằng máu và ý chí mới để đúc nên thanh thần kiếm phục hưng vương quốc.', 20, 100);

-- 2. Table motif_applicable_genres
INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f0013-0000-7000-8000-000000000015', '018f0013-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'fantasy'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f0013-0000-7000-8000-000000000016', '018f0013-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'wuxia'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

-- 3. Table state_rules
INSERT INTO state_rules (id, code, display_name) VALUES
  ('018f0013-0000-7000-8000-000000000029', 'heirloom_resonance_continuity', 'Quy tắc huyết mạch cộng hưởng với linh hồn thanh kiếm')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO state_rules (id, code, display_name) VALUES
  ('018f0013-0000-7000-8000-00000000002a', 'crucible_sacrifice_reforging', 'Quy tắc bắt buộc phải có chất xúc tác hy sinh khi tôi luyện lại')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

-- 4. Table states & state_down_by_rules
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0013-0000-7000-8000-000000000065', 'srb_flawed_ancestral_glory', '1. Thanh kiếm Gia truyền Khuyết tật (Flawed Ancestral Heritage)', 'Mang theo thanh kiếm của tổ tiên; danh tiếng vang dội nhưng ẩn chứa vết nứt định mệnh của thế hệ trước.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0013-0000-7000-8000-0000000000d2', '018f0013-0000-7000-8000-000000000065', (SELECT id FROM state_rules WHERE code = 'heirloom_resonance_continuity'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0013-0000-7000-8000-000000000066', 'srb_catastrophic_fracture', '2. Gãy vụn Dưới Búa rìu Kẻ thù (Catastrophic Fracture in Battle)', 'Đối đầu kẻ thù không đội trời chung; lưỡi kiếm gãy làm đôi, niềm tin sụp đổ, bóng tối bao trùm.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0013-0000-7000-8000-0000000000dc', '018f0013-0000-7000-8000-000000000066', (SELECT id FROM state_rules WHERE code = 'heirloom_resonance_continuity'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0013-0000-7000-8000-000000000067', 'srb_gathering_sacred_shards', '3. Nhặt từng Mảnh vỡ Đau thương (Gathering the Broken Shards)', 'Qùy gối nhặt từng mảnh thép sắc nhọn cứa vào tay; chấp nhận bài học thất bại xương máu.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0013-0000-7000-8000-0000000000e6', '018f0013-0000-7000-8000-000000000067', (SELECT id FROM state_rules WHERE code = 'heirloom_resonance_continuity'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0013-0000-7000-8000-000000000068', 'srb_crucible_of_reforging', '4. Lò lửa Tôi luyện & Hiến tế Máu (The Crucible of Sacred Reforging)', 'Nấu chảy mảnh kiếm trong ngọn lửa rồng hoặc dung nham thiêng; hòa tan máu và lời thề mới của chính mình.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0013-0000-7000-8000-0000000000f1', '018f0013-0000-7000-8000-000000000068', (SELECT id FROM state_rules WHERE code = 'crucible_sacrifice_reforging'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0013-0000-7000-8000-000000000069', 'srb_awakened_sun_blade', '5. Thần kiếm Tái sinh Sáng như Thái dương (The Awakened Unbreakable Blade)', 'Rút thanh kiếm mới ra khỏi lò đúc; đường vân phát sáng, chém đứt xiềng xích nô dịch bảo vệ nhân dân.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0013-0000-7000-8000-0000000000fb', '018f0013-0000-7000-8000-000000000069', (SELECT id FROM state_rules WHERE code = 'crucible_sacrifice_reforging'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

-- 5. Table components
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0013-0000-7000-8000-00000000012d', 'cmp_silver_hilt_crested_lion', 'Chuôi Kiếm Bạc Khắc Sư tử', 'Di vật truyền qua 7 đời vua nhưng mang lời nguyền thất trận.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0013-0000-7000-8000-00000000012e', 'cmp_metallic_snap_screech', 'Tiếng Kim loại Gãy Vang trời', 'Lưỡi thép văng tung tóe cắm vào đá lạnh khi bị chùy đen đập trúng.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0013-0000-7000-8000-00000000012f', 'cmp_bleeding_palms_shard_pouch', 'Bàn tay Nhuốm máu Bọc mảnh Thép', 'Cẩn thận bọc từng mảnh vỡ vào tấm lụa giữ sát bên tim.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0013-0000-7000-8000-000000000130', 'cmp_anvil_strike_dragon_fire', 'Nhát Búa Đập Dưới Lửa Rồng', 'Từng tiếng búa chan chát dội vào không gian hòa cùng mồ hôi và nước mắt.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0013-0000-7000-8000-000000000131', 'cmp_unsheathing_solar_flare', 'Rút Kiếm Phát Quang Rực rỡ', 'Lưỡi gươm bén ngót không vết xước xé toạc màn sương mù u tối.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

-- 6. Table emotional_ranges
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0013-0000-7000-8000-000000000191', 'emo_heavy_lineage_burden', 'Gánh nặng Huyết thống Nặng nề')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0013-0000-7000-8000-000000000192', 'emo_crushing_disillusionment', 'Sụp đổ Vỡ mộng')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0013-0000-7000-8000-000000000193', 'emo_penitent_reverence', 'Thành kính Ăn năn')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0013-0000-7000-8000-000000000194', 'emo_furnace_determination', 'Quyết tâm Lò lửa Nung nấu')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0013-0000-7000-8000-000000000195', 'emo_radiant_invincibility', 'Bất khả Chiến bại Rực sáng')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

-- 7. Table event_blocks & event_block_emotionals
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0013-0000-7000-8000-0000000001f5', 'eb_srb_heritage', '018f0013-0000-7000-8000-00000000012d', '018f0013-0000-7000-8000-000000000065', '018f0013-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0013-0000-7000-8000-000000000262', (SELECT id FROM emotional_ranges WHERE code = 'emo_heavy_lineage_burden'), '018f0013-0000-7000-8000-0000000001f5')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0013-0000-7000-8000-0000000001f6', 'eb_srb_fracture', '018f0013-0000-7000-8000-00000000012e', '018f0013-0000-7000-8000-000000000066', '018f0013-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0013-0000-7000-8000-00000000026d', (SELECT id FROM emotional_ranges WHERE code = 'emo_crushing_disillusionment'), '018f0013-0000-7000-8000-0000000001f6')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0013-0000-7000-8000-0000000001f7', 'eb_srb_shards', '018f0013-0000-7000-8000-00000000012f', '018f0013-0000-7000-8000-000000000067', '018f0013-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0013-0000-7000-8000-000000000278', (SELECT id FROM emotional_ranges WHERE code = 'emo_penitent_reverence'), '018f0013-0000-7000-8000-0000000001f7')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0013-0000-7000-8000-0000000001f8', 'eb_srb_reforge', '018f0013-0000-7000-8000-000000000130', '018f0013-0000-7000-8000-000000000068', '018f0013-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0013-0000-7000-8000-000000000283', (SELECT id FROM emotional_ranges WHERE code = 'emo_furnace_determination'), '018f0013-0000-7000-8000-0000000001f8')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0013-0000-7000-8000-0000000001f9', 'eb_srb_awakened', '018f0013-0000-7000-8000-000000000131', '018f0013-0000-7000-8000-000000000069', '018f0013-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0013-0000-7000-8000-00000000028e', (SELECT id FROM emotional_ranges WHERE code = 'emo_radiant_invincibility'), '018f0013-0000-7000-8000-0000000001f9')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

-- 8. Table event_transitions
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0013-0000-7000-8000-0000000002bd', 'Thảm bại gãy kiếm trong tuyệt vọng', '018f0013-0000-7000-8000-0000000001f5', '018f0013-0000-7000-8000-0000000001f6', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0013-0000-7000-8000-0000000002be', 'Gom nhặt mảnh vỡ đúc kết bài học', '018f0013-0000-7000-8000-0000000001f6', '018f0013-0000-7000-8000-0000000001f7', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0013-0000-7000-8000-0000000002bf', 'Thổi bùng ngọn lửa tôi rèn ý chí mới', '018f0013-0000-7000-8000-0000000001f7', '018f0013-0000-7000-8000-0000000001f8', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0013-0000-7000-8000-0000000002c0', 'Rút kiếm xuất thế định đoạt thiên hạ', '018f0013-0000-7000-8000-0000000001f8', '018f0013-0000-7000-8000-0000000001f9', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

-- 9. Table constraints & motif_constraints
INSERT INTO constraints (id, code, require, reason) VALUES
  ('018f0013-0000-7000-8000-000000000321', 'srb_ancestral_flaw_redemption', 'Thanh kiếm ban đầu bắt buộc phải mang một khiếm khuyết trong tư tưởng của tiền nhân được khắc phục khi đúc lại.', 'Tượng trưng cho sự tiến hóa của thế hệ sau vượt qua giới hạn của thế hệ trước.')
  ON CONFLICT (code) DO UPDATE SET require = EXCLUDED.require, reason = EXCLUDED.reason;

INSERT INTO motif_constraints (id, code, constraint_id, motif_id, syntax, constrain) VALUES
  ('018f0013-0000-7000-8000-000000000385', 'shattered_and_reforged_blade_srb_ancestral_flaw_redemption', (SELECT id FROM constraints WHERE code = 'srb_ancestral_flaw_redemption'), '018f0013-0000-7000-8000-000000000001', 'VALIDATE_MOTIF_RULE(''shattered_and_reforged_blade'', ''srb_ancestral_flaw_redemption'')', 'Thanh kiếm ban đầu bắt buộc phải mang một khiếm khuyết trong tư tưởng của tiền nhân được khắc phục khi đúc lại.')
  ON CONFLICT (code) DO UPDATE SET syntax = EXCLUDED.syntax, constrain = EXCLUDED.constrain;

INSERT INTO constraints (id, code, require, reason) VALUES
  ('018f0013-0000-7000-8000-000000000322', 'srb_reforging_catalyst', 'Khi đúc lại, bắt buộc phải có một kim loại mới hoặc một chất xúc tác tinh thần kết hợp với mảnh vỡ cũ.', 'Khẳng định thanh kiếm mới không chỉ đơn thuần là hàn gắn mà là sự chuyển hóa toàn diện.')
  ON CONFLICT (code) DO UPDATE SET require = EXCLUDED.require, reason = EXCLUDED.reason;

INSERT INTO motif_constraints (id, code, constraint_id, motif_id, syntax, constrain) VALUES
  ('018f0013-0000-7000-8000-000000000386', 'shattered_and_reforged_blade_srb_reforging_catalyst', (SELECT id FROM constraints WHERE code = 'srb_reforging_catalyst'), '018f0013-0000-7000-8000-000000000001', 'VALIDATE_MOTIF_RULE(''shattered_and_reforged_blade'', ''srb_reforging_catalyst'')', 'Khi đúc lại, bắt buộc phải có một kim loại mới hoặc một chất xúc tác tinh thần kết hợp với mảnh vỡ cũ.')
  ON CONFLICT (code) DO UPDATE SET syntax = EXCLUDED.syntax, constrain = EXCLUDED.constrain;

-- -----------------------------------------------------------------------------
-- 3. Narrator Configurations
-- -----------------------------------------------------------------------------
INSERT INTO narrator_configs
  (id, code, display_name, description, person, person_subtype, pov_scope, is_reliable, breaks_fourth_wall)
VALUES
  ('01a1bc44-fc00-7f57-8ce2-8ff270060e71', 'first_person_protagonist_single_pov_reliable_no_fourth_wall', 'narrator_config.first_person_protagonist_single_pov_reliable_no_fourth_wall.display_name', 'narrator_config.first_person_protagonist_single_pov_reliable_no_fourth_wall.description', 'FIRST_PERSON', 'PROTAGONIST', 'SINGLE_POV', 1, 0),
  ('01a1bc44-fc00-7ffa-97bf-2a8a03115cfd', 'first_person_protagonist_single_pov_reliable_fourth_wall', 'narrator_config.first_person_protagonist_single_pov_reliable_fourth_wall.display_name', 'narrator_config.first_person_protagonist_single_pov_reliable_fourth_wall.description', 'FIRST_PERSON', 'PROTAGONIST', 'SINGLE_POV', 1, 1),
  ('01a1bc44-fc00-7c7a-bce7-04b88608a1fb', 'first_person_protagonist_single_pov_unreliable_no_fourth_wall', 'narrator_config.first_person_protagonist_single_pov_unreliable_no_fourth_wall.display_name', 'narrator_config.first_person_protagonist_single_pov_unreliable_no_fourth_wall.description', 'FIRST_PERSON', 'PROTAGONIST', 'SINGLE_POV', 0, 0),
  ('01a1bc44-fc00-757f-bd3b-a4789d7375b8', 'first_person_protagonist_single_pov_unreliable_fourth_wall', 'narrator_config.first_person_protagonist_single_pov_unreliable_fourth_wall.display_name', 'narrator_config.first_person_protagonist_single_pov_unreliable_fourth_wall.description', 'FIRST_PERSON', 'PROTAGONIST', 'SINGLE_POV', 0, 1),
  ('01a1bc44-fc00-7b70-a28d-a648bca1cd06', 'first_person_protagonist_multiple_pov_reliable_no_fourth_wall', 'narrator_config.first_person_protagonist_multiple_pov_reliable_no_fourth_wall.display_name', 'narrator_config.first_person_protagonist_multiple_pov_reliable_no_fourth_wall.description', 'FIRST_PERSON', 'PROTAGONIST', 'MULTIPLE_POV', 1, 0),
  ('01a1bc44-fc00-714b-b1c7-803b23fdb707', 'first_person_protagonist_multiple_pov_reliable_fourth_wall', 'narrator_config.first_person_protagonist_multiple_pov_reliable_fourth_wall.display_name', 'narrator_config.first_person_protagonist_multiple_pov_reliable_fourth_wall.description', 'FIRST_PERSON', 'PROTAGONIST', 'MULTIPLE_POV', 1, 1),
  ('01a1bc44-fc00-7ae1-8777-ad6402042ade', 'first_person_protagonist_multiple_pov_unreliable_no_fourth_wall', 'narrator_config.first_person_protagonist_multiple_pov_unreliable_no_fourth_wall.display_name', 'narrator_config.first_person_protagonist_multiple_pov_unreliable_no_fourth_wall.description', 'FIRST_PERSON', 'PROTAGONIST', 'MULTIPLE_POV', 0, 0),
  ('01a1bc44-fc00-7541-baec-2421b4a64e43', 'first_person_protagonist_multiple_pov_unreliable_fourth_wall', 'narrator_config.first_person_protagonist_multiple_pov_unreliable_fourth_wall.display_name', 'narrator_config.first_person_protagonist_multiple_pov_unreliable_fourth_wall.description', 'FIRST_PERSON', 'PROTAGONIST', 'MULTIPLE_POV', 0, 1),
  ('01a1bc44-fc00-70b0-9916-46e93118a1ce', 'first_person_witness_peripheral_single_pov_reliable_no_fourth_wall', 'narrator_config.first_person_witness_peripheral_single_pov_reliable_no_fourth_wall.display_name', 'narrator_config.first_person_witness_peripheral_single_pov_reliable_no_fourth_wall.description', 'FIRST_PERSON', 'WITNESS_PERIPHERAL', 'SINGLE_POV', 1, 0),
  ('01a1bc44-fc00-7365-b90a-7d839aaaaf0f', 'first_person_witness_peripheral_single_pov_reliable_fourth_wall', 'narrator_config.first_person_witness_peripheral_single_pov_reliable_fourth_wall.display_name', 'narrator_config.first_person_witness_peripheral_single_pov_reliable_fourth_wall.description', 'FIRST_PERSON', 'WITNESS_PERIPHERAL', 'SINGLE_POV', 1, 1),
  ('01a1bc44-fc00-706d-b510-589cae684ccd', 'first_person_witness_peripheral_single_pov_unreliable_no_fourth_wall', 'narrator_config.first_person_witness_peripheral_single_pov_unreliable_no_fourth_wall.display_name', 'narrator_config.first_person_witness_peripheral_single_pov_unreliable_no_fourth_wall.description', 'FIRST_PERSON', 'WITNESS_PERIPHERAL', 'SINGLE_POV', 0, 0),
  ('01a1bc44-fc00-71ba-a6cc-48c61319a4c9', 'first_person_witness_peripheral_single_pov_unreliable_fourth_wall', 'narrator_config.first_person_witness_peripheral_single_pov_unreliable_fourth_wall.display_name', 'narrator_config.first_person_witness_peripheral_single_pov_unreliable_fourth_wall.description', 'FIRST_PERSON', 'WITNESS_PERIPHERAL', 'SINGLE_POV', 0, 1),
  ('01a1bc44-fc00-7a43-b61e-8ebd3f9cf8c2', 'first_person_witness_peripheral_multiple_pov_reliable_no_fourth_wall', 'narrator_config.first_person_witness_peripheral_multiple_pov_reliable_no_fourth_wall.display_name', 'narrator_config.first_person_witness_peripheral_multiple_pov_reliable_no_fourth_wall.description', 'FIRST_PERSON', 'WITNESS_PERIPHERAL', 'MULTIPLE_POV', 1, 0),
  ('01a1bc44-fc00-7871-bc58-b0618a8c2887', 'first_person_witness_peripheral_multiple_pov_reliable_fourth_wall', 'narrator_config.first_person_witness_peripheral_multiple_pov_reliable_fourth_wall.display_name', 'narrator_config.first_person_witness_peripheral_multiple_pov_reliable_fourth_wall.description', 'FIRST_PERSON', 'WITNESS_PERIPHERAL', 'MULTIPLE_POV', 1, 1),
  ('01a1bc44-fc00-7619-b046-8a5d8b6de639', 'first_person_witness_peripheral_multiple_pov_unreliable_no_fourth_wall', 'narrator_config.first_person_witness_peripheral_multiple_pov_unreliable_no_fourth_wall.display_name', 'narrator_config.first_person_witness_peripheral_multiple_pov_unreliable_no_fourth_wall.description', 'FIRST_PERSON', 'WITNESS_PERIPHERAL', 'MULTIPLE_POV', 0, 0),
  ('01a1bc44-fc00-7059-b3b6-48f3037e6e1f', 'first_person_witness_peripheral_multiple_pov_unreliable_fourth_wall', 'narrator_config.first_person_witness_peripheral_multiple_pov_unreliable_fourth_wall.display_name', 'narrator_config.first_person_witness_peripheral_multiple_pov_unreliable_fourth_wall.description', 'FIRST_PERSON', 'WITNESS_PERIPHERAL', 'MULTIPLE_POV', 0, 1),
  ('01a1bc44-fc00-79a6-a0f4-b15d9d977982', 'first_person_unreliable_first_single_pov_unreliable_no_fourth_wall', 'narrator_config.first_person_unreliable_first_single_pov_unreliable_no_fourth_wall.display_name', 'narrator_config.first_person_unreliable_first_single_pov_unreliable_no_fourth_wall.description', 'FIRST_PERSON', 'UNRELIABLE_FIRST', 'SINGLE_POV', 0, 0),
  ('01a1bc44-fc00-7ba0-bb7f-6806ee9cd785', 'first_person_unreliable_first_single_pov_unreliable_fourth_wall', 'narrator_config.first_person_unreliable_first_single_pov_unreliable_fourth_wall.display_name', 'narrator_config.first_person_unreliable_first_single_pov_unreliable_fourth_wall.description', 'FIRST_PERSON', 'UNRELIABLE_FIRST', 'SINGLE_POV', 0, 1),
  ('01a1bc44-fc00-7541-bbba-08729ac477b9', 'first_person_unreliable_first_multiple_pov_unreliable_no_fourth_wall', 'narrator_config.first_person_unreliable_first_multiple_pov_unreliable_no_fourth_wall.display_name', 'narrator_config.first_person_unreliable_first_multiple_pov_unreliable_no_fourth_wall.description', 'FIRST_PERSON', 'UNRELIABLE_FIRST', 'MULTIPLE_POV', 0, 0),
  ('01a1bc44-fc00-713b-ba44-b3d23ecf6c17', 'first_person_unreliable_first_multiple_pov_unreliable_fourth_wall', 'narrator_config.first_person_unreliable_first_multiple_pov_unreliable_fourth_wall.display_name', 'narrator_config.first_person_unreliable_first_multiple_pov_unreliable_fourth_wall.description', 'FIRST_PERSON', 'UNRELIABLE_FIRST', 'MULTIPLE_POV', 0, 1),
  ('01a1bc44-fc00-78ef-85bf-7a21ca202385', 'second_person_direct_address_single_pov_reliable_no_fourth_wall', 'narrator_config.second_person_direct_address_single_pov_reliable_no_fourth_wall.display_name', 'narrator_config.second_person_direct_address_single_pov_reliable_no_fourth_wall.description', 'SECOND_PERSON', 'DIRECT_ADDRESS', 'SINGLE_POV', 1, 0),
  ('01a1bc44-fc00-74b2-bcc8-c5cd75558364', 'second_person_direct_address_single_pov_reliable_fourth_wall', 'narrator_config.second_person_direct_address_single_pov_reliable_fourth_wall.display_name', 'narrator_config.second_person_direct_address_single_pov_reliable_fourth_wall.description', 'SECOND_PERSON', 'DIRECT_ADDRESS', 'SINGLE_POV', 1, 1),
  ('01a1bc44-fc00-7614-b325-7b51201e0fc6', 'second_person_direct_address_single_pov_unreliable_no_fourth_wall', 'narrator_config.second_person_direct_address_single_pov_unreliable_no_fourth_wall.display_name', 'narrator_config.second_person_direct_address_single_pov_unreliable_no_fourth_wall.description', 'SECOND_PERSON', 'DIRECT_ADDRESS', 'SINGLE_POV', 0, 0),
  ('01a1bc44-fc00-7708-8666-da0a213bb47a', 'second_person_direct_address_single_pov_unreliable_fourth_wall', 'narrator_config.second_person_direct_address_single_pov_unreliable_fourth_wall.display_name', 'narrator_config.second_person_direct_address_single_pov_unreliable_fourth_wall.description', 'SECOND_PERSON', 'DIRECT_ADDRESS', 'SINGLE_POV', 0, 1),
  ('01a1bc44-fc00-7685-ba62-42ce9fabd51f', 'second_person_direct_address_multiple_pov_reliable_no_fourth_wall', 'narrator_config.second_person_direct_address_multiple_pov_reliable_no_fourth_wall.display_name', 'narrator_config.second_person_direct_address_multiple_pov_reliable_no_fourth_wall.description', 'SECOND_PERSON', 'DIRECT_ADDRESS', 'MULTIPLE_POV', 1, 0),
  ('01a1bc44-fc00-7f7a-a3ef-657ef9f7e493', 'second_person_direct_address_multiple_pov_reliable_fourth_wall', 'narrator_config.second_person_direct_address_multiple_pov_reliable_fourth_wall.display_name', 'narrator_config.second_person_direct_address_multiple_pov_reliable_fourth_wall.description', 'SECOND_PERSON', 'DIRECT_ADDRESS', 'MULTIPLE_POV', 1, 1),
  ('01a1bc44-fc00-7d96-a80d-c63bbaa4e96d', 'second_person_direct_address_multiple_pov_unreliable_no_fourth_wall', 'narrator_config.second_person_direct_address_multiple_pov_unreliable_no_fourth_wall.display_name', 'narrator_config.second_person_direct_address_multiple_pov_unreliable_no_fourth_wall.description', 'SECOND_PERSON', 'DIRECT_ADDRESS', 'MULTIPLE_POV', 0, 0),
  ('01a1bc44-fc00-7604-8029-76e8827cb379', 'second_person_direct_address_multiple_pov_unreliable_fourth_wall', 'narrator_config.second_person_direct_address_multiple_pov_unreliable_fourth_wall.display_name', 'narrator_config.second_person_direct_address_multiple_pov_unreliable_fourth_wall.description', 'SECOND_PERSON', 'DIRECT_ADDRESS', 'MULTIPLE_POV', 0, 1),
  ('01a1bc44-fc00-7b2b-9f25-f6ba78b8a597', 'second_person_self_reflection_single_pov_reliable_no_fourth_wall', 'narrator_config.second_person_self_reflection_single_pov_reliable_no_fourth_wall.display_name', 'narrator_config.second_person_self_reflection_single_pov_reliable_no_fourth_wall.description', 'SECOND_PERSON', 'SELF_REFLECTION', 'SINGLE_POV', 1, 0),
  ('01a1bc44-fc00-748b-9f0e-dad40d856b89', 'second_person_self_reflection_single_pov_reliable_fourth_wall', 'narrator_config.second_person_self_reflection_single_pov_reliable_fourth_wall.display_name', 'narrator_config.second_person_self_reflection_single_pov_reliable_fourth_wall.description', 'SECOND_PERSON', 'SELF_REFLECTION', 'SINGLE_POV', 1, 1),
  ('01a1bc44-fc00-7dd2-9210-b84fd3333f28', 'second_person_self_reflection_single_pov_unreliable_no_fourth_wall', 'narrator_config.second_person_self_reflection_single_pov_unreliable_no_fourth_wall.display_name', 'narrator_config.second_person_self_reflection_single_pov_unreliable_no_fourth_wall.description', 'SECOND_PERSON', 'SELF_REFLECTION', 'SINGLE_POV', 0, 0),
  ('01a1bc44-fc00-7b6b-9bcd-e5c184a0f75c', 'second_person_self_reflection_single_pov_unreliable_fourth_wall', 'narrator_config.second_person_self_reflection_single_pov_unreliable_fourth_wall.display_name', 'narrator_config.second_person_self_reflection_single_pov_unreliable_fourth_wall.description', 'SECOND_PERSON', 'SELF_REFLECTION', 'SINGLE_POV', 0, 1),
  ('01a1bc44-fc00-7725-82cf-bc8619bd275c', 'second_person_self_reflection_multiple_pov_reliable_no_fourth_wall', 'narrator_config.second_person_self_reflection_multiple_pov_reliable_no_fourth_wall.display_name', 'narrator_config.second_person_self_reflection_multiple_pov_reliable_no_fourth_wall.description', 'SECOND_PERSON', 'SELF_REFLECTION', 'MULTIPLE_POV', 1, 0),
  ('01a1bc44-fc00-7669-a2dd-68a42ca93f38', 'second_person_self_reflection_multiple_pov_reliable_fourth_wall', 'narrator_config.second_person_self_reflection_multiple_pov_reliable_fourth_wall.display_name', 'narrator_config.second_person_self_reflection_multiple_pov_reliable_fourth_wall.description', 'SECOND_PERSON', 'SELF_REFLECTION', 'MULTIPLE_POV', 1, 1),
  ('01a1bc44-fc00-7aa2-84b2-1ad0cb5e8166', 'second_person_self_reflection_multiple_pov_unreliable_no_fourth_wall', 'narrator_config.second_person_self_reflection_multiple_pov_unreliable_no_fourth_wall.display_name', 'narrator_config.second_person_self_reflection_multiple_pov_unreliable_no_fourth_wall.description', 'SECOND_PERSON', 'SELF_REFLECTION', 'MULTIPLE_POV', 0, 0),
  ('01a1bc44-fc00-7625-abf2-52f5000fd82e', 'second_person_self_reflection_multiple_pov_unreliable_fourth_wall', 'narrator_config.second_person_self_reflection_multiple_pov_unreliable_fourth_wall.display_name', 'narrator_config.second_person_self_reflection_multiple_pov_unreliable_fourth_wall.description', 'SECOND_PERSON', 'SELF_REFLECTION', 'MULTIPLE_POV', 0, 1),
  ('01a1bc44-fc00-70f9-89fb-e526c112a435', 'second_person_interactive_single_pov_reliable_no_fourth_wall', 'narrator_config.second_person_interactive_single_pov_reliable_no_fourth_wall.display_name', 'narrator_config.second_person_interactive_single_pov_reliable_no_fourth_wall.description', 'SECOND_PERSON', 'INTERACTIVE', 'SINGLE_POV', 1, 0),
  ('01a1bc44-fc00-7445-8447-93080aa7a2df', 'second_person_interactive_single_pov_reliable_fourth_wall', 'narrator_config.second_person_interactive_single_pov_reliable_fourth_wall.display_name', 'narrator_config.second_person_interactive_single_pov_reliable_fourth_wall.description', 'SECOND_PERSON', 'INTERACTIVE', 'SINGLE_POV', 1, 1),
  ('01a1bc44-fc00-744f-b8ff-8aad2eadeba3', 'second_person_interactive_single_pov_unreliable_no_fourth_wall', 'narrator_config.second_person_interactive_single_pov_unreliable_no_fourth_wall.display_name', 'narrator_config.second_person_interactive_single_pov_unreliable_no_fourth_wall.description', 'SECOND_PERSON', 'INTERACTIVE', 'SINGLE_POV', 0, 0),
  ('01a1bc44-fc00-7649-8428-c014ade93163', 'second_person_interactive_single_pov_unreliable_fourth_wall', 'narrator_config.second_person_interactive_single_pov_unreliable_fourth_wall.display_name', 'narrator_config.second_person_interactive_single_pov_unreliable_fourth_wall.description', 'SECOND_PERSON', 'INTERACTIVE', 'SINGLE_POV', 0, 1),
  ('01a1bc44-fc00-75c7-91a1-c11b1a86c29a', 'second_person_interactive_multiple_pov_reliable_no_fourth_wall', 'narrator_config.second_person_interactive_multiple_pov_reliable_no_fourth_wall.display_name', 'narrator_config.second_person_interactive_multiple_pov_reliable_no_fourth_wall.description', 'SECOND_PERSON', 'INTERACTIVE', 'MULTIPLE_POV', 1, 0),
  ('01a1bc44-fc00-7f9e-8e97-4cf8da4b299c', 'second_person_interactive_multiple_pov_reliable_fourth_wall', 'narrator_config.second_person_interactive_multiple_pov_reliable_fourth_wall.display_name', 'narrator_config.second_person_interactive_multiple_pov_reliable_fourth_wall.description', 'SECOND_PERSON', 'INTERACTIVE', 'MULTIPLE_POV', 1, 1),
  ('01a1bc44-fc00-7f2b-93f5-55f2cc3cb8ff', 'second_person_interactive_multiple_pov_unreliable_no_fourth_wall', 'narrator_config.second_person_interactive_multiple_pov_unreliable_no_fourth_wall.display_name', 'narrator_config.second_person_interactive_multiple_pov_unreliable_no_fourth_wall.description', 'SECOND_PERSON', 'INTERACTIVE', 'MULTIPLE_POV', 0, 0),
  ('01a1bc44-fc00-7260-81f3-80f31442970c', 'second_person_interactive_multiple_pov_unreliable_fourth_wall', 'narrator_config.second_person_interactive_multiple_pov_unreliable_fourth_wall.display_name', 'narrator_config.second_person_interactive_multiple_pov_unreliable_fourth_wall.description', 'SECOND_PERSON', 'INTERACTIVE', 'MULTIPLE_POV', 0, 1),
  ('01a1bc44-fc00-7889-8be0-59c4ab1dd452', 'third_person_omniscient_shifting_omniscient_reliable_no_fourth_wall', 'narrator_config.third_person_omniscient_shifting_omniscient_reliable_no_fourth_wall.display_name', 'narrator_config.third_person_omniscient_shifting_omniscient_reliable_no_fourth_wall.description', 'THIRD_PERSON', 'OMNISCIENT', 'SHIFTING_OMNISCIENT', 1, 0),
  ('01a1bc44-fc00-72db-9faa-69647b028b0a', 'third_person_omniscient_shifting_omniscient_reliable_fourth_wall', 'narrator_config.third_person_omniscient_shifting_omniscient_reliable_fourth_wall.display_name', 'narrator_config.third_person_omniscient_shifting_omniscient_reliable_fourth_wall.description', 'THIRD_PERSON', 'OMNISCIENT', 'SHIFTING_OMNISCIENT', 1, 1),
  ('01a1bc44-fc00-7b8d-af2d-1c8622d6b25c', 'third_person_omniscient_shifting_omniscient_unreliable_no_fourth_wall', 'narrator_config.third_person_omniscient_shifting_omniscient_unreliable_no_fourth_wall.display_name', 'narrator_config.third_person_omniscient_shifting_omniscient_unreliable_no_fourth_wall.description', 'THIRD_PERSON', 'OMNISCIENT', 'SHIFTING_OMNISCIENT', 0, 0),
  ('01a1bc44-fc00-7722-abf8-a442b13e983c', 'third_person_omniscient_shifting_omniscient_unreliable_fourth_wall', 'narrator_config.third_person_omniscient_shifting_omniscient_unreliable_fourth_wall.display_name', 'narrator_config.third_person_omniscient_shifting_omniscient_unreliable_fourth_wall.description', 'THIRD_PERSON', 'OMNISCIENT', 'SHIFTING_OMNISCIENT', 0, 1),
  ('01a1bc44-fc00-7643-9cea-ffa43c0d94d9', 'third_person_limited_single_pov_reliable_no_fourth_wall', 'narrator_config.third_person_limited_single_pov_reliable_no_fourth_wall.display_name', 'narrator_config.third_person_limited_single_pov_reliable_no_fourth_wall.description', 'THIRD_PERSON', 'LIMITED', 'SINGLE_POV', 1, 0),
  ('01a1bc44-fc00-788e-8313-4433f3ef575d', 'third_person_limited_single_pov_reliable_fourth_wall', 'narrator_config.third_person_limited_single_pov_reliable_fourth_wall.display_name', 'narrator_config.third_person_limited_single_pov_reliable_fourth_wall.description', 'THIRD_PERSON', 'LIMITED', 'SINGLE_POV', 1, 1),
  ('01a1bc44-fc00-7e52-9b7d-56cf47f9ab69', 'third_person_limited_single_pov_unreliable_no_fourth_wall', 'narrator_config.third_person_limited_single_pov_unreliable_no_fourth_wall.display_name', 'narrator_config.third_person_limited_single_pov_unreliable_no_fourth_wall.description', 'THIRD_PERSON', 'LIMITED', 'SINGLE_POV', 0, 0),
  ('01a1bc44-fc00-7911-a9fd-56a3c953a668', 'third_person_limited_single_pov_unreliable_fourth_wall', 'narrator_config.third_person_limited_single_pov_unreliable_fourth_wall.display_name', 'narrator_config.third_person_limited_single_pov_unreliable_fourth_wall.description', 'THIRD_PERSON', 'LIMITED', 'SINGLE_POV', 0, 1),
  ('01a1bc44-fc00-75c9-a15d-bbe38907102c', 'third_person_limited_multiple_pov_reliable_no_fourth_wall', 'narrator_config.third_person_limited_multiple_pov_reliable_no_fourth_wall.display_name', 'narrator_config.third_person_limited_multiple_pov_reliable_no_fourth_wall.description', 'THIRD_PERSON', 'LIMITED', 'MULTIPLE_POV', 1, 0),
  ('01a1bc44-fc00-74ad-aeff-191bb96a7495', 'third_person_limited_multiple_pov_reliable_fourth_wall', 'narrator_config.third_person_limited_multiple_pov_reliable_fourth_wall.display_name', 'narrator_config.third_person_limited_multiple_pov_reliable_fourth_wall.description', 'THIRD_PERSON', 'LIMITED', 'MULTIPLE_POV', 1, 1),
  ('01a1bc44-fc00-7d3b-8185-0fea8d199745', 'third_person_limited_multiple_pov_unreliable_no_fourth_wall', 'narrator_config.third_person_limited_multiple_pov_unreliable_no_fourth_wall.display_name', 'narrator_config.third_person_limited_multiple_pov_unreliable_no_fourth_wall.description', 'THIRD_PERSON', 'LIMITED', 'MULTIPLE_POV', 0, 0),
  ('01a1bc44-fc00-79fe-a522-f8b955112e3e', 'third_person_limited_multiple_pov_unreliable_fourth_wall', 'narrator_config.third_person_limited_multiple_pov_unreliable_fourth_wall.display_name', 'narrator_config.third_person_limited_multiple_pov_unreliable_fourth_wall.description', 'THIRD_PERSON', 'LIMITED', 'MULTIPLE_POV', 0, 1),
  ('01a1bc44-fc00-7baa-b5c4-8dc968286223', 'third_person_objective_single_pov_reliable_no_fourth_wall', 'narrator_config.third_person_objective_single_pov_reliable_no_fourth_wall.display_name', 'narrator_config.third_person_objective_single_pov_reliable_no_fourth_wall.description', 'THIRD_PERSON', 'OBJECTIVE', 'SINGLE_POV', 1, 0),
  ('01a1bc44-fc00-794c-8b3a-09a4c48bd1a4', 'third_person_objective_single_pov_reliable_fourth_wall', 'narrator_config.third_person_objective_single_pov_reliable_fourth_wall.display_name', 'narrator_config.third_person_objective_single_pov_reliable_fourth_wall.description', 'THIRD_PERSON', 'OBJECTIVE', 'SINGLE_POV', 1, 1),
  ('01a1bc44-fc00-79b5-9874-3d6c28440d24', 'third_person_objective_single_pov_unreliable_no_fourth_wall', 'narrator_config.third_person_objective_single_pov_unreliable_no_fourth_wall.display_name', 'narrator_config.third_person_objective_single_pov_unreliable_no_fourth_wall.description', 'THIRD_PERSON', 'OBJECTIVE', 'SINGLE_POV', 0, 0),
  ('01a1bc44-fc00-73c2-9ac2-c95baf83dc00', 'third_person_objective_single_pov_unreliable_fourth_wall', 'narrator_config.third_person_objective_single_pov_unreliable_fourth_wall.display_name', 'narrator_config.third_person_objective_single_pov_unreliable_fourth_wall.description', 'THIRD_PERSON', 'OBJECTIVE', 'SINGLE_POV', 0, 1),
  ('01a1bc44-fc00-729d-9dbc-b7492a68e3be', 'third_person_objective_multiple_pov_reliable_no_fourth_wall', 'narrator_config.third_person_objective_multiple_pov_reliable_no_fourth_wall.display_name', 'narrator_config.third_person_objective_multiple_pov_reliable_no_fourth_wall.description', 'THIRD_PERSON', 'OBJECTIVE', 'MULTIPLE_POV', 1, 0),
  ('01a1bc44-fc00-7b82-9634-77a2ff558bdc', 'third_person_objective_multiple_pov_reliable_fourth_wall', 'narrator_config.third_person_objective_multiple_pov_reliable_fourth_wall.display_name', 'narrator_config.third_person_objective_multiple_pov_reliable_fourth_wall.description', 'THIRD_PERSON', 'OBJECTIVE', 'MULTIPLE_POV', 1, 1),
  ('01a1bc44-fc00-7d89-ba35-dbfda5532d62', 'third_person_objective_multiple_pov_unreliable_no_fourth_wall', 'narrator_config.third_person_objective_multiple_pov_unreliable_no_fourth_wall.display_name', 'narrator_config.third_person_objective_multiple_pov_unreliable_no_fourth_wall.description', 'THIRD_PERSON', 'OBJECTIVE', 'MULTIPLE_POV', 0, 0),
  ('01a1bc44-fc00-70bc-94f2-99d35394ebd4', 'third_person_objective_multiple_pov_unreliable_fourth_wall', 'narrator_config.third_person_objective_multiple_pov_unreliable_fourth_wall.display_name', 'narrator_config.third_person_objective_multiple_pov_unreliable_fourth_wall.description', 'THIRD_PERSON', 'OBJECTIVE', 'MULTIPLE_POV', 0, 1)
ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name, description = EXCLUDED.description, person = EXCLUDED.person, person_subtype = EXCLUDED.person_subtype, pov_scope = EXCLUDED.pov_scope, is_reliable = EXCLUDED.is_reliable, breaks_fourth_wall = EXCLUDED.breaks_fourth_wall;

-- -----------------------------------------------------------------------------
-- 4. Narrative Directives
-- -----------------------------------------------------------------------------

-- 1.1. Category: PROSE_PRINCIPLE (Nguyên tắc hành văn)
INSERT INTO narrative_directives (id, code, description, category, is_system) VALUES
('0196ba10-0001-7000-8000-000000000001', 'show_dont_tell', 'Thể hiện cảm xúc, tính cách nhân vật và diễn biến thông qua hành động vật lý, phản ứng cơ thể, đối thoại và chi tiết giác quan thay vì dùng người kể chuyện tóm tắt, dán nhãn trừu tượng.', 'PROSE_PRINCIPLE', true),
('0196ba10-0001-7000-8000-000000000002', 'sensory_grounding', 'Neo giữ phân cảnh vào thực tại bằng cách kích hoạt chọn lọc 5 giác quan (thị giác, thính giác, khứu giác, xúc giác, vị giác) gắn liền mật thiết với trải nghiệm tức thời của nhân vật điểm nhìn.', 'PROSE_PRINCIPLE', true),
('0196ba10-0001-7000-8000-000000000003', 'action_driven_emotion', 'Thay thế các từ ngữ chỉ cảm xúc chung chung (sợ hãi, tức giận, đau buồn) bằng phản ứng sinh lý cụ thể (tim đập dồn dập, đồng tử co thắt, khô khốc cổ họng, siết chặt nắm tay).', 'PROSE_PRINCIPLE', true),
('0196ba10-0002-7000-8000-000000000001', 'deep_pov_immersion', 'Hòa nhập hoàn toàn giọng văn trần thuật vào nhận thức, tâm lý, thái độ và vốn từ vựng của nhân vật điểm nhìn (Deep POV), xóa nhòa ranh giới giữa tác giả và nhân vật.', 'PROSE_PRINCIPLE', true),
('0196ba10-0002-7000-8000-000000000002', 'internal_monologue_weave', 'Đan cài dòng suy nghĩ nội tâm trực tiếp của nhân vật vào dòng chảy hành động một cách tự nhiên mà không cần dùng thẻ dẫn rườm rà (như ''anh nghĩ'', ''cô tự nhủ'') hay in nghiêng gượng ép.', 'PROSE_PRINCIPLE', true),
('0196ba10-0003-7000-8000-000000000001', 'active_voice_preference', 'Ưu tiên sử dụng câu chủ động với động từ hành động mạnh mẽ, tạo lực đẩy trực diện và nhịp điệu dứt khoát cho từng phân cảnh.', 'PROSE_PRINCIPLE', true),
('0196ba10-0003-7000-8000-000000000002', 'sentence_rhythm_variation', 'Biến hóa linh hoạt độ dài và cấu trúc câu: dùng chuỗi câu ngắn, cô đọng khi đẩy cao trào kịch tính và dùng câu phức có nhịp điệu khi miêu tả chiều sâu tâm cảnh.', 'PROSE_PRINCIPLE', true),
('0196ba10-0003-7000-8000-000000000003', 'specific_concrete_nouns', 'Sử dụng danh từ và động từ cụ thể, chính xác, giàu hình ảnh thị giác thay vì lạm dụng danh từ trừu tượng phải đi kèm nhiều tính từ giải thích.', 'PROSE_PRINCIPLE', true),
('0196ba10-0004-7000-8000-000000000001', 'clarity_first', 'Đặt sự trong sáng, rõ nghĩa và chuẩn xác lên hàng đầu; đảm bảo độc giả hình dung được ngay lập tức không gian, phương vị và hành động mà không phải khựng lại đọc lại.', 'PROSE_PRINCIPLE', true),
('0196ba10-0004-7000-8000-000000000002', 'fresh_metaphor_precision', 'Sáng tạo các hình ảnh so sánh và ẩn dụ độc đáo, bén nhọn, gắn liền với thế giới quan và cảm xúc nhân vật thay vì trang trí câu từ rỗng tuếch.', 'PROSE_PRINCIPLE', true),
('0196ba10-0005-7000-8000-000000000004', 'lyrical_economy', 'Kết hợp hài hòa giữa chất thơ, nhạc tính của câu chữ với tính kinh tế của ngôn từ (lyrical economy), giữ cho câu văn gợi cảm nhưng không làm trì trệ mạch tự sự.', 'PROSE_PRINCIPLE', true),
('0196ba10-0006-7000-8000-000000000002', 'positive_form_statements', 'Diễn đạt tư tưởng và sự kiện ở dạng khẳng định trực diện; hạn chế cách diễn đạt vòng vèo, phủ định của phủ định làm yếu đi lực tác động của câu văn.', 'PROSE_PRINCIPLE', true),
('0196ba10-0006-7000-8000-000000000004', 'emphatic_sentence_endings', 'Bố trí từ ngữ mang thông tin trọng tâm hoặc tạo điểm rơi cảm xúc mạnh nhất ở cuối câu hoặc cuối đoạn văn để tối đa hóa ấn tượng tâm lý.', 'PROSE_PRINCIPLE', true),
('0196ba10-0007-7000-8000-000000000003', 'contextual_dialogue_clarity', 'Để bản thân nội dung câu thoại, ngữ cảnh và hành động kèm theo tự thể hiện ngữ khí, cảm xúc của người nói thay vì phụ thuộc vào thẻ thoại giải thích.', 'PROSE_PRINCIPLE', true),
('0196ba10-0007-7000-8000-000000000004', 'fearless_direct_prose', 'Viết trực diện, can trường và dứt khoát; không rào đón, thanh minh hay dài dòng vì sợ độc giả không cảm nhận được ngụ ý.', 'PROSE_PRINCIPLE', true),
('0196ba10-0008-7000-8000-000000000005', 'clarity_over_dogma', 'Sẵn sàng linh hoạt phá vỡ các quy tắc ngữ pháp hay tu từ cứng nhắc khi điều đó đem lại câu văn tự nhiên, giàu sức sống và chân thực hơn.', 'PROSE_PRINCIPLE', true),
('0196ba10-0009-7000-8000-000000000001', 'invisible_authorial_style', 'Giữ giọng văn trần thuật tự nhiên, trong suốt (invisible style); nếu câu văn có vẻ đang ''phô diễn kỹ thuật viết văn'', hãy chỉnh sửa cho đến khi nó hòa nhập trơn tru vào câu chuyện.', 'PROSE_PRINCIPLE', true),
('0196ba10-000a-7000-8000-000000000001', 'windowpane_prose', 'Áp dụng phong cách ''kính cửa sổ trong suốt'' (windowpane prose theo Brandon Sanderson): câu chữ rõ ràng, không màu mè để độc giả nhìn xuyên qua từ ngữ vào thẳng cốt truyện.', 'PROSE_PRINCIPLE', true),
('0196ba10-000c-7000-8000-000000000003', 'pacing_cadence_modulation', 'Điều tiết nhịp điệu tự sự luân phiên khoa học giữa các trường đoạn hành động dồn dập (câu ngắn, nhịp nhanh) và các khoảng nghỉ phản tư (sequel scenes) để người đọc lắng đọng cảm xúc.', 'PROSE_PRINCIPLE', true),
('0196ba10-000e-7000-8000-000000000003', 'voice_authenticity_bias', 'Thể hiện tính cách và thiên kiến nhận thức của nhân vật ngay trong cách lựa chọn vốn từ, ngữ điệu và thái độ miêu tả sự vật xung quanh.', 'PROSE_PRINCIPLE', true),
('0196ba10-0011-7000-8000-000000000004', 'conflict_embedded_exposition', 'Cài cắm thông tin bối cảnh trực tiếp vào các cuộc tranh luận nảy lửa, xung đột quyền lợi hoặc các tình thế nan giải mà nhân vật đang nỗ lực tháo gỡ.', 'PROSE_PRINCIPLE', true),
('0196ba10-0012-7000-8000-000000000001', 'scene_transition_economy', 'Chuyển cảnh sắc bén và kinh tế: bước vào cảnh muộn nhất có thể (khi xung đột đã chớm nở) và rời cảnh sớm nhất có thể (ngay khi đạt được bước ngoặt cảm xúc).', 'PROSE_PRINCIPLE', true);

-- 1.2. Category: RESTRAINT (Tiết chế)
INSERT INTO narrative_directives (id, code, description, category, is_system) VALUES
('0196ba10-0001-7000-8000-000000000004', 'telling_summary_restraint', 'Tiết chế tối đa văn kể lể tóm tắt (telling summary) tại các phân cảnh cao trào; chỉ sử dụng tóm tắt khi cần chuyển cảnh lướt qua khoảng thời gian chết.', 'RESTRAINT', true),
('0196ba10-0002-7000-8000-000000000003', 'filter_word_restraint', 'Loại bỏ triệt để các từ lọc tạo khoảng cách nhận thức (như: thấy, nghe, nhận thấy, cảm thấy, dường như, tự hỏi) để người đọc trực tiếp trải nghiệm thế giới qua giác quan nhân vật.', 'RESTRAINT', true),
('0196ba10-0002-7000-8000-000000000004', 'authorial_intrusion_restraint', 'Kiềm chế sự can thiệp của tác giả (authorial intrusion); không chèn triết lý hay bình luận mang tính thượng đẳng vượt ngoài nhận thức của nhân vật điểm nhìn.', 'RESTRAINT', true),
('0196ba10-0003-7000-8000-000000000004', 'passive_voice_restraint', 'Tiết chế cấu trúc câu bị động; chỉ sử dụng khi cần nhấn mạnh đối tượng bị tác động hoặc khi kẻ thực hiện hành vi cố tình bị giấu kín trong bóng tối.', 'RESTRAINT', true),
('0196ba10-0003-7000-8000-000000000005', 'cliche_avoidance', 'Tuyệt đối tránh sử dụng các ẩn dụ sáo mòn, thành ngữ cũ rích (clichés) hoặc các mô tả rập khuôn làm giảm giá trị nghệ thuật của trang viết.', 'RESTRAINT', true),
('0196ba10-0004-7000-8000-000000000003', 'overwriting_restraint', 'Tiết chế lối viết dài dòng thừa thãi (overwriting); cắt bỏ các mệnh đề giải thích lại những gì đã được bộc lộ rõ qua hành động và ngữ cảnh đối thoại.', 'RESTRAINT', true),
('0196ba10-0004-7000-8000-000000000004', 'melodrama_restraint', 'Kiềm chế cảm xúc cường điệu hóa (melodrama), tránh gượng ép giọt nước mắt hay lạm dụng ngôn từ bi thương quá đà; để sức nặng tự nhiên của sự kiện chạm tới người đọc.', 'RESTRAINT', true),
('0196ba10-0005-7000-8000-000000000001', 'purple_prose_restraint', 'Kiên quyết tiết chế văn phong hoa mỹ diêm dúa (purple prose): loại bỏ câu từ cầu kỳ rườm rà, từ vựng đa âm bóng bẩy làm đứt gãy sự tập trung vào mạch truyện.', 'RESTRAINT', true),
('0196ba10-0005-7000-8000-000000000002', 'adjective_pileup_restraint', 'Tránh xếp chồng nhiều tính từ và trạng từ liên tiếp trước một danh từ/động từ; nếu có hai tính từ đồng nghĩa, hãy bỏ cả hai và chọn một danh từ thật mạnh.', 'RESTRAINT', true),
('0196ba10-0005-7000-8000-000000000003', 'thesaurus_syndrome_restraint', 'Tránh hội chứng lạm dụng từ điển đồng nghĩa (thesaurus syndrome); không dùng từ cổ, từ hiếm một cách gượng ép khi từ phổ thông diễn đạt chính xác hơn.', 'RESTRAINT', true),
('0196ba10-0006-7000-8000-000000000001', 'omit_needless_words', 'Cắt bỏ mọi từ ngữ thừa thãi (Omit needless words theo Strunk & White): một câu không nên thừa từ vô ích và một đoạn không nên chứa câu vô duyên.', 'RESTRAINT', true),
('0196ba10-0006-7000-8000-000000000005', 'loose_sentence_restraint', 'Tiết chế chuỗi câu ghép dài lỏng lẻo nối liên tiếp bằng các liên từ ''và'', ''nhưng'', ''vì vậy'' gây cảm giác đơn điệu, ru ngủ.', 'RESTRAINT', true),
('0196ba10-0007-7000-8000-000000000001', 'adverb_restraint', 'Tiết chế tối đa trạng từ chỉ cách thức đi kèm động từ (theo Stephen King: trạng từ là biểu hiện của sự bất lực trong việc chọn động từ chính xác).', 'RESTRAINT', true),
('0196ba10-0007-7000-8000-000000000002', 'dialogue_tag_restraint', 'Tiết chế các thẻ thoại cầu kỳ, hoa mỹ (như ''anh ta giận dữ gầm lên'', ''cô ta thổn thức than thở''); ưu tiên dùng thẻ thoại vô hình hoặc hành động kèm theo.', 'RESTRAINT', true),
('0196ba10-0008-7000-8000-000000000001', 'dying_metaphor_restraint', 'Không bao giờ sử dụng những hình ảnh tu từ, phép so sánh quen mòn đã xuất hiện tràn lan trên sách báo thông thường (theo quy tắc George Orwell).', 'RESTRAINT', true),
('0196ba10-0008-7000-8000-000000000002', 'short_word_preference', 'Ưu tiên dùng từ ngữ ngắn gọn, gãy gọn; không bao giờ dùng từ dài dòng hoa mỹ khi từ ngắn có thể diễn đạt trọn vẹn và sáng nghĩa.', 'RESTRAINT', true),
('0196ba10-0008-7000-8000-000000000003', 'radical_word_cutting', 'Nếu có thể cắt bỏ bớt một từ trong câu mà không làm suy giảm ý nghĩa, hãy luôn luôn kiên quyết cắt bỏ từ đó.', 'RESTRAINT', true),
('0196ba10-0008-7000-8000-000000000004', 'jargon_loanword_restraint', 'Tiết chế từ ngữ kỹ thuật chuyên ngành (jargon), thuật ngữ học thuật hoặc từ ngoại lai xa lạ khi đã có từ đời thường tương đương.', 'RESTRAINT', true),
('0196ba10-0009-7000-8000-000000000002', 'skip_boring_parts', 'Lược bỏ những phần nhàm chán mà độc giả có xu hướng lướt qua (theo Elmore Leonard): không tả thời tiết mở đầu dông dài, tả tĩnh vật vô thưởng vô phạt hay đường đi nước bước tẻ nhạt.', 'RESTRAINT', true),
('0196ba10-0009-7000-8000-000000000003', 'exclamation_point_restraint', 'Kiểm soát nghiêm ngặt dấu chấm than (!); không dùng dấu câu để cưỡng ép tạo kịch tính giả tạo cho lời thoại và câu văn.', 'RESTRAINT', true),
('0196ba10-000a-7000-8000-000000000005', 'worldbuilding_infodump_restraint', 'Tuyệt đối tiết chế việc xả hàng khối thông tin bối cảnh lịch sử, địa lý hay cơ chế phép thuật (infodump) tách rời khỏi hành động của nhân vật.', 'RESTRAINT', true),
('0196ba10-000b-7000-8000-000000000003', 'telegraphed_twist_restraint', 'Tiết chế việc gợi mở quá lộ liễu (over-telegraphing) trước cú ngoặt cốt truyện; giữ cho các manh mối ẩn mình tự nhiên dưới mục đích bề nổi của cảnh.', 'RESTRAINT', true),
('0196ba10-000c-7000-8000-000000000004', 'premature_exposition_restraint', 'Kiềm chế việc giải thích nguyên nhân quá sớm; hãy để người đọc tự đặt câu hỏi và khao khát tìm câu trả lời trước khi cung cấp lời giải đáp.', 'RESTRAINT', true),
('0196ba10-000d-7000-8000-000000000003', 'omniscient_spoiler_restraint', 'Khi sử dụng góc nhìn toàn tri tạo kịch tính, tiết chế việc tiết lộ toàn bộ kết cục để vẫn duy trì được câu hỏi nhân vật sẽ ứng biến ra sao.', 'RESTRAINT', true),
('0196ba10-000e-7000-8000-000000000004', 'cheap_lie_restraint', 'Tiết chế việc để người kể chuyện nói dối trắng trợn vô căn cứ với độc giả; sự che giấu thông tin phải bắt nguồn từ động cơ tâm lý hoặc chấn thương của nhân vật.', 'RESTRAINT', true),
('0196ba10-000f-7000-8000-000000000003', 'unfired_detail_restraint', 'Tiết chế việc miêu tả tỉ mỉ các đồ vật, đạo cụ hoặc chi tiết nhân vật phụ nếu chúng không mang lại giá trị cốt truyện, chủ đề hay cảm xúc nào về sau.', 'RESTRAINT', true),
('0196ba10-0010-7000-8000-000000000003', 'dead_end_sub_plot_restraint', 'Tiết chế việc kéo dài manh mối giả (red herring) quá lâu mà không đóng góp cho sự phát triển nhân vật hay hé lộ thông tin phụ nào, tránh gây cảm giác bị lừa rẻ tiền.', 'RESTRAINT', true),
('0196ba10-0011-7000-8000-000000000001', 'infodump_restraint', 'Tuyệt đối ngăn chặn việc tạm dừng mạch truyện để thuyết minh bách khoa toàn thư về thiết lập thế giới hay quá khứ nhân vật.', 'RESTRAINT', true),
('0196ba10-0011-7000-8000-000000000002', 'as_you_know_bob_restraint', 'Tránh triệt để lối thoại gượng gạo ''Như anh đã biết đấy...'' (As You Know, Bob), nơi hai nhân vật giảng giải cho nhau nghe những điều mà cả hai vốn đều đã biết rõ.', 'RESTRAINT', true),
('0196ba10-0012-7000-8000-000000000003', 'tangent_subplot_restraint', 'Tiết chế việc sa đà vào các nhánh cốt truyện phụ hoặc hồi ức quá khứ (flashbacks) lan man làm loãng xung đột trọng tâm và làm chùng nhịp chương.', 'RESTRAINT', true);

-- 1.3. Category: INFORMATION_CONTROL (Kiểm soát thông tin)
INSERT INTO narrative_directives (id, code, description, category, is_system) VALUES
('0196ba10-0001-7000-8000-000000000005', 'subtextual_emotion_reveal', 'Kiểm soát luồng cảm xúc ngầm (subtext): để độc giả tự nhận biết mâu thuẫn tâm lý nhân vật thông qua độ chênh lệch giữa lời nói dối bề ngoài và hành vi vô thức bên trong.', 'INFORMATION_CONTROL', true),
('0196ba10-0002-7000-8000-000000000005', 'pov_epistemic_boundary', 'Giới hạn nghiêm ngặt tầm nhìn thông tin trong phạm vi những gì nhân vật điểm nhìn trực tiếp quan sát, cảm nhận hoặc suy đoán hợp lý tại thời điểm sự kiện diễn ra.', 'INFORMATION_CONTROL', true),
('0196ba10-0009-7000-8000-000000000004', 'sparse_physical_description', 'Tiết chế liệt kê trang phục và dung mạo từ đầu đến chân; chỉ hé lộ 1-2 đặc điểm nhận dạng nổi bật mang tính biểu tượng để độc giả tự hoàn thiện hình dung.', 'INFORMATION_CONTROL', true),
('0196ba10-0009-7000-8000-000000000005', 'prologue_backstory_restraint', 'Tránh mở đầu bằng chương dẫn nhập (prologue) giải thích lịch sử dài dòng; ném nhân vật vào biến cố trực diện và hé lộ quá khứ nhỏ giọt khi thực sự cần thiết.', 'INFORMATION_CONTROL', true),
('0196ba10-000a-7000-8000-000000000002', 'earned_exposition_grounding', 'Chỉ cung cấp quy tắc thế giới hoặc cơ chế sức mạnh khi thông tin đó trực tiếp giải quyết vấn đề trước mắt hoặc tạo thành chướng ngại thực tế cho nhân vật.', 'INFORMATION_CONTROL', true),
('0196ba10-000a-7000-8000-000000000003', 'layered_foreshadowing', 'Gieo mầm phục bút (foreshadowing) theo nhiều tầng tinh tế từ sớm, để khi cú ngoặt bùng nổ, độc giả cảm thấy ngỡ ngàng nhưng hoàn toàn hợp lý và thỏa đáng.', 'INFORMATION_CONTROL', true),
('0196ba10-000a-7000-8000-000000000004', 'promise_progress_payoff_pacing', 'Điều tiết nhịp độ thông tin theo chu trình Lời hứa (Promise) -> Tiến triển (Progress) -> Đền đáp (Payoff), bảo đảm mỗi phân cảnh đều tạo ra sự dịch chuyển thông tin.', 'INFORMATION_CONTROL', true),
('0196ba10-000b-7000-8000-000000000001', 'information_gap_tension', 'Duy trì sức hút trang sách bằng cách chủ động tạo ra các khoảng trống thông tin then chốt và kiểm soát tốc độ lấp đầy câu trả lời cho người đọc.', 'INFORMATION_CONTROL', true),
('0196ba10-000b-7000-8000-000000000002', 'escalation_via_revelation', 'Mỗi lần hé lộ một sự thật mới (revelation) phải đóng vai trò như một đòn bẩy nâng cao cọc cược hiểm nguy hoặc làm đảo lộn cục diện của nhân vật.', 'INFORMATION_CONTROL', true),
('0196ba10-000c-7000-8000-000000000001', 'revelation_over_infodump', 'Thay thế các đoạn thuyết minh tĩnh bằng chuỗi khám phá động thông qua quá trình điều tra, va chạm và lần mò dấu vết của nhân vật.', 'INFORMATION_CONTROL', true),
('0196ba10-000c-7000-8000-000000000002', 'delayed_reveal_suspense', 'Trì hoãn có chủ đích thời điểm tung ra câu trả lời (delayed reveal) bằng cách cài cắm trở ngại hợp lý để kéo căng cảm giác hồi hộp chờ đợi.', 'INFORMATION_CONTROL', true),
('0196ba10-000d-7000-8000-000000000001', 'dramatic_irony_asymmetry', 'Tạo ra sự bất cân xứng thông tin có chủ đích (Dramatic Irony): cho độc giả biết trước hiểm họa hoặc bí mật mà nhân vật trong cảnh hoàn toàn chưa hay biết.', 'INFORMATION_CONTROL', true),
('0196ba10-000d-7000-8000-000000000002', 'anticipation_over_surprise', 'Ưu tiên xây dựng cảm giác thắc thỏm âu lo (anticipation/dread) thông qua sự bất đối xứng thông tin thay vì chỉ dựa vào những cú giật mình bất ngờ chốc lát.', 'INFORMATION_CONTROL', true),
('0196ba10-000e-7000-8000-000000000001', 'unreliable_perspective_filtering', 'Kiểm soát thông tin qua lăng kính người kể chuyện không đáng tin: che giấu, bóp méo sự thật thông qua định kiến, chấn thương hoặc sự tự lừa dối của nhân vật.', 'INFORMATION_CONTROL', true),
('0196ba10-000e-7000-8000-000000000002', 'breadcrumbs_of_contradiction', 'Cài cắm các chi tiết mâu thuẫn khách quan nhỏ để người đọc tinh ý nhận ra sự lệch chuẩn giữa lời trần thuật của nhân vật chính và thực tế diễn ra.', 'INFORMATION_CONTROL', true),
('0196ba10-000f-7000-8000-000000000001', 'chekhovs_gun_setup', 'Áp dụng nguyên lý khẩu súng của Chekhov: mọi chi tiết, bảo vật hoặc kỹ năng được nhấn mạnh ở hồi đầu bắt buộc phải được kích hoạt và giải quyết vấn đề ở hồi sau.', 'INFORMATION_CONTROL', true),
('0196ba10-000f-7000-8000-000000000002', 'camouflage_setup_in_scene', 'Ngụy trang chi tiết phục bút (Chekhov''s Gun) ẩn bên dưới một mục đích thông thường khác của phân cảnh để độc giả không sớm nhận ra dụng ý sắp đặt.', 'INFORMATION_CONTROL', true),
('0196ba10-0010-7000-8000-000000000001', 'red_herring_misdirection', 'Sử dụng kỹ thuật tung hỏa mù (Red Herring): cài cắm manh mối giả và đối tượng khả nghi hợp lý để che chắn cho đầu mối chân chính đang hiện diện.', 'INFORMATION_CONTROL', true),
('0196ba10-0010-7000-8000-000000000002', 'mystery_clue_pacing', 'Phân bổ nhịp độ hé lộ manh mối thật và manh mối giả đan xen có trật tự, duy trì trạng thái suy luận và lật mở giả thuyết liên tục của người đọc.', 'INFORMATION_CONTROL', true),
('0196ba10-0011-7000-8000-000000000003', 'iceberg_worldbuilding_control', 'Áp dụng nguyên lý tảng băng trôi (Iceberg Theory): chỉ phơi bày phần nổi gắn trực tiếp với cốt truyện, để phần chìm đồ sộ của thế giới tự toát lên qua chi tiết nhỏ.', 'INFORMATION_CONTROL', true),
('0196ba10-0012-7000-8000-000000000002', 'atmospheric_foreshadowing', 'Sử dụng không khí, thanh âm và sự biến đổi của bối cảnh không gian để dự báo ngầm về thảm họa sắp giáng xuống trước khi sự việc thực sự bùng nổ.', 'INFORMATION_CONTROL', true);

-- Expose Phase 0 reference catalogs and project genre linking to the chat model.
-- Only tools with a valid object schema and a prompt_block_tools link are composed.
UPDATE tools SET
  display_name = 'search_genre',
  description = 'Tra cứu thể loại theo mã hoặc tên hiển thị trong danh mục. Dùng mã code trả về khi gắn thể loại vào dự án.',
  input_schema = '{"type":"object","properties":{"query":{"type":"string","description":"Từ khóa mã hoặc tên thể loại; bỏ qua để liệt kê."},"limit":{"type":"integer","minimum":1,"maximum":50}},"additionalProperties":false}'
WHERE code = 'search_genre';

UPDATE tools SET
  display_name = 'search_motif',
  description = 'Tra cứu mô-típ theo mã hoặc tên hiển thị, có thể lọc theo nhóm. Chỉ dùng mã đã tra cứu khi gán mô-típ nhân vật.',
  input_schema = '{"type":"object","properties":{"query":{"type":"string","description":"Từ khóa mã hoặc tên mô-típ; bỏ qua để liệt kê."},"category":{"type":"string","enum":["character_arc","relationship","symbolic","plot"]},"limit":{"type":"integer","minimum":1,"maximum":50}},"additionalProperties":false}'
WHERE code = 'search_motif';

UPDATE tools SET
  display_name = 'add_project_genre',
  description = 'Gắn thể loại đã xác minh từ search_genre vào dự án; genre_code phải là mã danh mục có thật. declared_sources.genre_code xác định nguồn của liên kết.',
  input_schema = '{"type":"object","properties":{"genre_code":{"type":"string","minLength":1},"is_primary":{"type":"boolean"},"declared_sources":{"type":"object","properties":{"genre_code":{"type":"string","enum":["user","ai_generated"]}},"required":["genre_code"],"additionalProperties":false}},"required":["genre_code","declared_sources"],"additionalProperties":false}'
WHERE code = 'add_project_genre';

INSERT OR IGNORE INTO prompt_block_tools (id, prompt_block_id, tool_id)
SELECT '1b5d6e21-7f90-4e86-8c31-9df51c17b001', p.id, t.id
FROM prompt_blocks p JOIN tools t ON t.code = 'search_genre'
WHERE p.prompt_code = 'tool_library';

INSERT OR IGNORE INTO prompt_block_tools (id, prompt_block_id, tool_id)
SELECT '1b5d6e21-7f90-4e86-8c31-9df51c17b002', p.id, t.id
FROM prompt_blocks p JOIN tools t ON t.code = 'search_motif'
WHERE p.prompt_code = 'tool_library';

INSERT OR IGNORE INTO prompt_block_tools (id, prompt_block_id, tool_id)
SELECT '1b5d6e21-7f90-4e86-8c31-9df51c17b003', p.id, t.id
FROM prompt_blocks p JOIN tools t ON t.code = 'add_project_genre'
WHERE p.prompt_code = 'tool_library';

-- Preserve existing schema properties and only expand the array-path choices.
UPDATE tools SET input_schema = json_set(input_schema,
  '$.properties.array_path.enum', json('["require.character","require.project.genres"]'))
WHERE code = 'get_array_element_fields' AND json_valid(input_schema);

-- Optional target tolerances are nonnegative integers, including zero.
UPDATE tools SET input_schema = json_set(input_schema,
  '$.properties.target.properties.arc_tolerance', json('{"type":"integer","minimum":0,"maximum":4294967295}'),
  '$.properties.target.properties.chapter_tolerance', json('{"type":"integer","minimum":0,"maximum":4294967295}'),
  '$.properties.target.properties.word_per_chapter_tolerance', json('{"type":"integer","minimum":0,"maximum":4294967295}'))
WHERE code = 'set_project' AND json_valid(input_schema);

-- Seed the per-phase planning and writing prompts (`core.md` §2, §3.8–§3.9).
-- Each block is composed together with the shared blocks (introduction, invariant_rule,
-- planning_mode, exception_handling, out_of_scope, tool_library, story_context); tool access is
-- granted only through `prompt_block_tools`, so a phase without rows would carry no tools.

INSERT INTO prompt_blocks (id, prompt_code, display_name, heading_level, content_format, content)
VALUES
('01a1bfb1-80f2-7ba2-b06e-aa78246e26b2', 'planning_phase_0', 'Phase 0: Cốt lõi bắt buộc', 2, 'MARKDOWN',
 'Phase 0 thu thập điều kiện mở khóa tại `require`, nhưng bóc tách và ghi qua tool mọi thông tin người dùng cung cấp cho bất kỳ phase nào ngay trong lượt hiện tại; thông tin không có trường đích trong schema ghi vào `user_constraints`. Không sinh nội dung phái sinh của phase chưa mở. Kiểm tra `require.project`: `display_name`, `summary`, `language` (ngôn ngữ sáng tác, tách biệt `system_language` dùng để giao tiếp), `target.arc_count`, `target.chapter_count`, `target.word_per_chapter_count` đều lớn hơn 0, và `genres` có ít nhất một thể loại mang `code`. `require.character[]` phải có ít nhất một phần tử; mỗi nhân vật cần `display_name` hoặc `nickname`, `biological_sex` và `motif_code`. Giữ trường chưa được người dùng xác định là `null` hoặc mảng rỗng. Không tự suy ra số Arc, số chương, định mức chữ, tên hay thể loại; mọi giá trị AI đề xuất hoặc suy luận, đặc biệt `display_name`, phải được trình bày và được người dùng xác nhận trước khi gọi tool ghi. Giá trị người dùng đã nêu rõ được ghi qua tool với nguồn `user`; tôn trọng quy tắc `pending_patches` khi đụng dữ liệu đã bảo vệ. Nếu thông tin mới mâu thuẫn State, nêu độ lệch và hỏi người dùng, không tự sửa. Khi hỏi bổ sung trường thiếu, gọi tool đọc trường `null` hoặc tool đọc trường trong mảng tương ứng; hỏi đúng tối đa ba câu hỏi cố định mà tool trả về theo `system_language`, không tự thêm câu hỏi. Chỉ mở Draft Building khi toàn bộ `require` đã hoàn chỉnh.'),

('01a1bfb1-80f3-702d-b7fa-4e25222f1c15', 'planning_phase_1', 'Phase 1: Tổng quan dàn ý Arc', 2, 'MARKDOWN',
 'Mục tiêu: hoàn thiện `optional.overview_phase.arcs[]` ở mức vĩ mô, đúng `require.project.target.arc_count` Arc. Trước khi viết, đọc State hiện tại và danh mục `motif`, `theme`; không tự tạo mã. Với mỗi Arc, chuẩn bị: `position` (1..N), `display_name`, `summary` nêu biến cố chính, `motif.code`, `motif.event_blocks[]` (khoảng `start_chapter`–`end_chapter` nằm trong phạm vi Arc, tóm tắt khối biến cố), và `themes` gồm `question`, `resolution` (loại + nội dung), `message`, cùng `arguments[]` với `role` `PRIMARY`/`ANTITHESIS`/`CAUTIONARY`/`WITNESS`. Dùng tool `add_arc`/`set_arc`/`set_arc_motif`/`add_arc_event_block`/`add_theme`/`add_theme_argument` theo từng thực thể. Tên Arc, motif, theme và mọi nội dung sáng tác phái sinh phải được đề xuất cụ thể và chờ người dùng xác nhận trước khi ghi; không ghi màn hình trống hoặc giữ chỗ. Nếu người dùng nêu thay đổi `arc_count`/`chapter_count`, nêu lý do và chờ xác nhận, không tự đổi target. Sau khi hoàn thành, đối chiếu lại: mỗi Arc có tên, tóm tắt, motif và ít nhất một event block; báo rõ phần còn thiếu thay vì tuyên bố hoàn tất.'),

('01a1bfb1-80f3-710b-b877-dd9704138f25', 'planning_phase_2', 'Phase 2: Dàn ý chương và cảnh thô', 2, 'MARKDOWN',
 'Mục tiêu: rã từng Arc thành `chapters[]` và `scenes[]` ở mức dàn ý, tổng số chương bằng `require.project.target.chapter_count`. `chapter.position` là thứ tự duy nhất trên toàn bộ dự án, chạy xuyên Arc; `scene.position` là thứ tự trong chương. Mỗi chương cần `display_name` và `summary`; mỗi cảnh cần `display_name`, `summary` (diễn biến và xung đột). Chưa tự gán `world_setting_code` nếu cảnh thô chưa xác định địa điểm; trường này được hoàn thiện ở Phase 4 và bắt buộc trước commit. Dùng `add_chapter`/`set_chapter`/`add_scene`/`set_scene`; khi cần đổi cấu trúc lớn, trình bày kế hoạch phân bổ chương theo Arc và chờ xác nhận. Không tự bịa tên chương hàng loạt: với tên và tóm tắt do AI đề xuất, nêu rõ và chờ người dùng duyệt trước khi ghi. Sau khi xong, kiểm tra tổng số chương và báo lệch nếu có.'),

('01a1bfb1-80f3-7f79-9da4-97211dd8db3e', 'planning_phase_3', 'Phase 3: Bối cảnh thế giới', 2, 'MARKDOWN',
 'Mục tiêu: lập cây `world_settings` với `code`, `parent_code`, tên, loại (`LocationType`), mô tả, điều kiện tiếp cận (`LocationAccess`) và các `lore_entries` gắn vào bối cảnh. Gốc là đơn vị rộng nhất (REALM/WORLD), nút con cụ thể dần; mỗi cảnh phải chọn nút sâu nhất mà tóm tắt cảnh xác định được. `lore_entries.data` là JSON object theo `LoreType` (`CULTURE`/`HISTORY`/`FACTION`/`ITEM`); key chưa biết để vắng mặt, không tự biến thành mảng rỗng. Dùng `add_world_setting`/`set_world_setting`/`add_lore_entry`/`set_lore_entry`, ghi cha trước con. Mọi tên địa danh, tổ chức, vật phẩm hoặc lore do AI đề xuất phải được người dùng xác nhận trước khi ghi. Không xóa hoặc gán lại địa điểm mà người dùng đã gán cho cảnh.'),

('01a1bfb1-80f3-7b9d-8548-53da097c9ffe', 'planning_phase_4', 'Phase 4: Kỹ thuật phân cảnh', 2, 'MARKDOWN',
 'Mục tiêu: với từng cảnh đã có, xác định `style_preset_code`, `narrator_config_code`, `world_setting_code`, danh sách lore được sử dụng (`lore_entry_code`, `usage`, `reveal_to_reader`, `reveal_to_pov`) và các nhân vật có mặt cùng `is_pov`. Đối chiếu catalog bằng tool đọc (`search_style_preset`, `search_narrator_config`, `search_character`, `get_state`) trước khi chọn code; không tự bịa mã. Cảnh ngôi kể giới hạn có đúng một nhân vật `is_pov`; ngôi kể toàn tri/khách quan có không nhân vật `is_pov`. Địa điểm phải tồn tại, thuộc cùng dự án và đủ cụ thể theo tóm tắt cảnh; nếu cần tạo bối cảnh mới, trình bày đề xuất để người dùng xác nhận trước khi ghi. Không tự đổi địa điểm người dùng đã gán. Dùng `set_scene_config`, `add_scene_character`, `add_scene_lore_entry`.'),

('01a1bfb1-80f5-7cf8-b189-183b1c1f4adf', 'planning_phase_5', 'Phase 5: Hồ sơ nhân vật chuyên sâu', 2, 'MARKDOWN',
 'Mục tiêu: hoàn thiện ba sub-phase theo thứ tự 5.1 ngoại hình (`character_appearance_phase`), 5.2 động cơ và tâm lý (`character_psychological_phase`), 5.3 ký ức và năng lực (`character_life_experience_phase`). Mỗi sub-phase chạy trên toàn bộ nhân vật trong `require.character[]`, giữ tên hiển thị hoặc biệt danh nhất quán với Phase 0. 5.1: `species`, `gender`, chiều cao/cân nặng kèm đơn vị, `hair`, `skin`, `eyes`, `outfit_style`, `distinguishing_marks[]`, `default_summary`. 5.2: `core_trauma_and_belief`, `cognitive_moral_framework`, `narrative_engine`, `behavioral_adaptation`. 5.3: `memories[]` (cảnh nguồn có thể null) và `competences[]` (bắt buộc `scene_code` trỏ cảnh đã tồn tại; nếu chưa biết cảnh, hỏi người dùng, không tự chọn). Dùng `add_character_appearance`/`set_character_appearance`, `add_character_psychological`/`set_character_psychological`, `add_character_memory`, `add_character_competence`, `add_competence_capability`/`add_competence_limitation`. Mọi đặc điểm, tên năng lực hoặc `display_name` do AI sáng tạo phải được nêu rõ và xác nhận trước khi ghi.'),

('01a1bfb1-80f5-7567-a72e-bc4627c509a9', 'planning_phase_6', 'Phase 6: Động lực quan hệ', 2, 'MARKDOWN',
 'Mục tiêu: mô tả quan hệ có hướng giữa các nhân vật qua cặp `from_code` và `to_code`, vai trò xã hội (`relationship_roles`) và trạng thái cảm xúc theo cảnh. Lưu thưa các chiều `trust`, `affection`, `hostility`, `fear`, `respect`, `familiarity`, `dependency`: chỉ ghi tại cảnh có thay đổi hoặc keyframe; dùng enum `RelationDimensionLevel` (`NONE`/`LOW`/`MODERATE`/`HIGH`/`ABSOLUTE`), không dùng thang số tự đặt. Kiểm tra nhân vật và cảnh tham chiếu đã tồn tại trước khi ghi; không tự tạo nhân vật. Dùng `add_relationship`/`set_relationship`, `add_relationship_role`, `set_relationship_state`. Mọi quan hệ hoặc tên do AI đề xuất cần người dùng xác nhận trước khi ghi.'),

('01a1bfb1-80f5-7412-8e0b-9df894a0934b', 'writing_chapter', 'Writing Mode: Viết một chương đã chọn', 2, 'MARKDOWN',
 'Chỉ bắt đầu khi plan đã commit và người dùng chọn một chương. Trong một phiên chỉ viết nội dung thuộc chương đó, không tự chuyển chương hoặc viết hàng loạt. Chỉ sinh nội dung cho cảnh đang mở khóa và thuộc `EditScope` của request đang hoạt động; cảnh đang khóa thì không sửa, không generate, và không tự mở khóa hộ người dùng. Kết quả sinh vào buffer tạm để người dùng Review/Accept; chỉ Save cấp chương mới ghi nội dung và `scene_versions` xuống DB. AI không đổi `is_locked`, không ghi thẳng `scenes.content` hoặc `scene_versions`. Giữ continuity theo cảnh: đối chiếu nhân vật, năng lực, ký ức, quan hệ, địa điểm, lore và diễn biến đã xảy ra; dữ liệu trùng giữa các cảnh không phải lỗi, chỉ mâu thuẫn logic mới là cảnh báo và cảnh báo continuity không tự chặn Save. Nếu cần bổ sung tên, chi tiết tình tiết hoặc giá trị chưa được người dùng chấp thuận, nêu rõ đề xuất và chờ xác nhận trước khi lưu.');

-- Planning phases share the full entity tool surface (`core.md` §1.4.9); UI-only actions and
-- Writing Mode mutations are never granted to planning prompts.
INSERT INTO prompt_block_tools (id, prompt_block_id, tool_id)
SELECT
  lower(hex(randomblob(4))) || '-' || lower(hex(randomblob(2))) || '-' ||
  lower(hex(randomblob(2))) || '-' || lower(hex(randomblob(2))) || '-' || lower(hex(randomblob(6))),
  pb.id, t.id
FROM prompt_blocks pb
JOIN tools t
WHERE pb.prompt_code IN (
    'planning_phase_0', 'planning_phase_1', 'planning_phase_2',
    'planning_phase_3', 'planning_phase_4', 'planning_phase_5', 'planning_phase_6'
  )
  AND t.code NOT IN (
    'commit_draft', 'confirm_phase', 'confirm_patch', 'reject_patch', 'confirm_all_patches',
    'restore_version', 'lock_scene', 'unlock_scene', 'write_scene_content', 'add_scene_version',
    'write_phase_report', 'read_phase_reports'
  );

-- Phase 3 subagents additionally read/write temporary probe reports.
INSERT INTO prompt_block_tools (id, prompt_block_id, tool_id)
SELECT
  lower(hex(randomblob(4))) || '-' || lower(hex(randomblob(2))) || '-' ||
  lower(hex(randomblob(2))) || '-' || lower(hex(randomblob(2))) || '-' || lower(hex(randomblob(6))),
  pb.id, t.id
FROM prompt_blocks pb
JOIN tools t
WHERE pb.prompt_code = 'planning_phase_3'
  AND t.code IN ('write_phase_report', 'read_phase_reports');

-- Writing Mode gets read/context tools only; scene content is produced through the
-- generation stream and Save Chapter, never through direct write tools.
INSERT INTO prompt_block_tools (id, prompt_block_id, tool_id)
SELECT
  lower(hex(randomblob(4))) || '-' || lower(hex(randomblob(2))) || '-' ||
  lower(hex(randomblob(2))) || '-' || lower(hex(randomblob(2))) || '-' || lower(hex(randomblob(6))),
  pb.id, t.id
FROM prompt_blocks pb
JOIN tools t
WHERE pb.prompt_code = 'writing_chapter'
  AND t.code IN ('get_state', 'lookup_outline', 'get_tool_definition', 'search_character', 'get_field_null');

-- Ensure every granted planning tool carries a callable JSON schema. Write handlers validate the
-- real field names, so the schema is intentionally permissive: it lets the model emit the
-- documented parameters without the backend inventing per-tool required lists.
UPDATE tools
SET input_schema = '{"type":"object","additionalProperties":true,"properties":{}}'
WHERE input_schema IS NULL;

-- Every character the author names must exist in require.character before any phase references
-- them. Phase 0 creates them; later phases must add missing ones instead of silently dropping.

UPDATE prompt_blocks
SET content = content || ' Bổ sung bắt buộc: nếu người dùng đã nêu tên bất kỳ nhân vật nào — kể cả nhân vật phụ, người thân, đồng nghiệp, phản diện — thì tạo đủ từng nhân vật bằng `add_character` với `source = user` ngay trong lượt; không chỉ giữ hai nhân vật chính. Với nhân vật chỉ được mô tả mà chưa có tên, đề xuất tên và chờ người dùng xác nhận trước khi ghi; không tự đặt tên. Mỗi nhân vật cần `display_name` hoặc `nickname`, `biological_sex`, `motif_code`; thiếu trường nào thì hỏi đúng trường đó. Liệt kê lại toàn bộ nhân vật đã ghi để người dùng đối chiếu.'
WHERE prompt_code = 'planning_phase_0';

UPDATE prompt_blocks
SET content = content || ' Bổ sung bắt buộc: kiểm tra mọi nhân vật được nhắc trong tóm tắt Arc/theme; nếu chưa có trong `require.character` thì gọi `add_character` trước khi dùng làm luận điểm hoặc tham chiếu. Không bỏ sót nhân vật nào đã được người dùng nêu tên.'
WHERE prompt_code IN ('planning_phase_1', 'planning_phase_2');

UPDATE prompt_blocks
SET content = content || ' Bổ sung bắt buộc: với mỗi cảnh, liệt kê đầy đủ nhân vật có mặt; nếu một nhân vật chưa tồn tại trong `require.character`, gọi `add_character` trước rồi mới gắn vào cảnh. Không bỏ qua nhân vật phụ.'
WHERE prompt_code IN ('planning_phase_4', 'planning_phase_6');

-- The character-coverage addendum must name the real tool parameter; `source` is not accepted.
UPDATE prompt_blocks
SET content = replace(content,
  'tạo đủ từng nhân vật bằng `add_character` với `source = user`',
  'tạo đủ từng nhân vật bằng `add_character` và khai báo `declared_sources` cho từng trường (ví dụ {"display_name":"user","nickname":"user","biological_sex":"user","motif_code":"user"}); không truyền tham số `source`')
WHERE prompt_code = 'planning_phase_0';

-- Phase 6 must create missing characters instead of refusing to.
UPDATE prompt_blocks
SET content = replace(content,
  'Kiểm tra nhân vật và cảnh tham chiếu đã tồn tại trước khi ghi; không tự tạo nhân vật.',
  'Kiểm tra nhân vật và cảnh tham chiếu đã tồn tại trước khi ghi; nếu người dùng đã nêu tên một nhân vật chưa có trong require.character, gọi add_character trước rồi mới ghi quan hệ. Không tự đặt tên mới khi người dùng chưa xác nhận.')
WHERE prompt_code = 'planning_phase_6';

-- Seed the theme catalog from docs/temp_seed/system/theme.md (12 parent + 73 work themes).
-- Idempotent: ON CONFLICT (code) DO UPDATE keeps existing rows and never re-issues ids.

-- Tham chiếu nghiên cứu thị trường: `docs/temp_seed/report/subagent_d.md`, `subagent_e.md`, `subagent_f.md`.

-- Nhóm 1 · Chủ đề mẹ hệ thống (12 dòng, `parent_theme_id` NULL)
INSERT INTO themes (id, code, parent_theme_id, question, resolution_type, resolution, message, created_at, updated_at) VALUES
-- Chủ đề mẹ: Vòng xoáy thù hận · Toàn cầu · nguồn + [Cycle of Revenge Trope and Retributive Cycle in Fiction](https://tvtropes.org/pmwiki/pmwiki.php/Main/CycleOfRevenge)
-- PRIMARY: Kẻ Tha Thứ (ACTION) - Chấp nhận gánh chịu tổn thương để cắt đứt xích thù hận · ANTITHESIS: Kẻ Báo Thù Cực Đoan (VERBAL) - Tuyên xưng nợ máu phải trả bằng máu · CAUTIONARY: Kẻ Khởi Xướng Chiến Tranh (OUTCOME) - Bị chính ngọn lửa thù hận mình châm ngòi thiêu rụi · WITNESS: Sử Gia / Kẻ Sống Sót (IMPLICIT) - Ghi chép lại đống tro tàn của các thế hệ diệt vong
  ('018f0a00-0001-7000-8000-000000000001', 'THEME_PARENT_CYCLE_OF_HATRED', NULL, 'Liệu bạo lực và thù hận có thể bị hóa giải bằng sự thấu hiểu nhân loại, hay nó là bản chất tự hủy tất yếu của lịch sử?', 'OPEN', 'Vòng xoáy bạo lực chỉ có thể tạm dừng khi các bên chấp nhận đối thoại và từ bỏ tư duy trả đũa nhãn tiền, nhưng nguy cơ tái bùng phát luôn tiềm ẩn theo chu kỳ thời gian.', 'Sự thấu cảm và năng lực tha thứ là chiếc neo duy nhất giữ cho nhân loại không trượt dài vào vực thẳm diệt vong.', datetime('now'), datetime('now')),
-- Chủ đề mẹ: Ý chí tự do và định mệnh · Toàn cầu · nguồn + [Sliding Scale of Free Will vs Fate in Literature](https://tvtropes.org/pmwiki/pmwiki.php/Main/SlidingScaleOfFreeWillVsFate)
-- PRIMARY: Kẻ Kháng Lệnh Thiên Mệnh (ACTION) - Chiến đấu đến cùng để tự viết nên câu chuyện đời mình · ANTITHESIS: Nhà Thuyết Quyết Định Luận (VERBAL) - Quy phục tuyệt đối trước dòng chảy số phận định sẵn · CAUTIONARY: Kẻ Trốn Chạy Định Mệnh (OUTCOME) - Càng cố né tránh sấm truyền thì hành động lại càng dẫn thẳng tới sấm truyền đó · WITNESS: Tiên Tri / Nhà Chiêm Tinh (IMPLICIT) - Quan sát sự vùng vẫy vô vọng của con người trước quy luật vũ trụ
  ('018f0a00-0001-7000-8000-000000000002', 'THEME_PARENT_FREEDOM_VS_DETERMINISM', NULL, 'Ý chí tự do của con người có thực sự hiện hữu, hay ta chỉ là những con rối cử động theo sự sắp đặt vô hình của định mệnh và cấu trúc thế giới?', 'AMBIGUOUS', 'Con người không thể kiểm soát điểm khởi đầu hay biến cố khách quan, nhưng tự do chân chính nằm ở quyền năng lựa chọn thái độ và phẩm giá đạo đức khi đối mặt với nghịch cảnh.', 'Phẩm giá cao quý nhất của sinh mệnh là dám lựa chọn và chịu trách nhiệm về số phận của chính mình ngay cả khi biết trước cái chết.', datetime('now'), datetime('now')),
-- Chủ đề mẹ: Tha hóa quyền lực · Toàn cầu · nguồn + [Abuse of Power and Corruption Theme Analysis](https://www.litcharts.com/lit/the-house-of-the-scorpion/themes/abuse-of-power-and-corruption)
-- PRIMARY: Kẻ Chối Từ Vương Miện (ACTION) - Sẵn sàng buông bỏ quyền lực để giữ gìn sự thanh sạch của tâm hồn · ANTITHESIS: Bạo Chúa Độc Tài (VERBAL) - Tin rằng quyền lực tối thượng là phương tiện duy nhất để duy trì trật tự và thực thi công lý · CAUTIONARY: Kẻ Cải Cách Tha Hóa (OUTCOME) - Khởi đầu bằng lý tưởng cứu thế cao đẹp nhưng dần trở thành chính con quái vật mình từng muốn tiêu diệt · WITNESS: Lương Tri Nhân Dân (IMPLICIT) - Quần chúng chịu đựng hậu quả của những cuộc tranh đoạt quyền lực thượng tầng
  ('018f0a00-0001-7000-8000-000000000003', 'THEME_PARENT_POWER_AND_CORRUPTION', NULL, 'Quyền lực tuyệt đối có tất yếu dẫn tới sự tha hóa tuyệt đối, và cái giá phải trả khi dùng bạo lực để áp đặt trật tự là gì?', 'RESOLVED', 'Không có ai miễn nhiễm trước sự tha hóa của quyền lực; bất kỳ trật tự nào được xây dựng trên sự cưỡng đoạt và đàn áp đều gieo sẵn mầm mống sụp đổ từ bên trong.', 'Mục đích cao đẹp không thể biện minh cho phương tiện tàn bạo; sức mạnh chỉ có ý nghĩa khi nó phục vụ bảo vệ sự sống và tự do của tha nhân.', datetime('now'), datetime('now')),
-- Chủ đề mẹ: Sinh tồn và nhân tính · Toàn cầu · nguồn + [Survival vs. Morality Theme in The Dark Forest](https://www.litcharts.com/lit/the-dark-forest/themes/survival-vs-morality)
-- PRIMARY: Kẻ Giữ Gìn Ngọn Đuốc Đạo Đức (ACTION) - Thà hy sinh bản thân chứ không chà đạp lên sinh mệnh đồng loại · ANTITHESIS: Kẻ Darwin Xã Hội Cực Đoan (VERBAL) - Xem đạo đức là ảo tưởng yếu đuối cản trở quy luật sinh tồn của kẻ mạnh · CAUTIONARY: Kẻ Thực Dụng Lạc Lối (OUTCOME) - Bảo toàn được thể xác nhờ thủ đoạn nhưng linh hồn bị hủy hoại vĩnh viễn bởi sự ghê tởm chính mình · WITNESS: Kẻ Yếu Thế Cầu Cứu (IMPLICIT) - Nạn nhân phản chiếu bản chất đạo đức thực sự của kẻ đang nắm giữ quyền sinh sát
  ('018f0a00-0001-7000-8000-000000000004', 'THEME_PARENT_HUMANITY_VS_SURVIVAL', NULL, 'Trong một môi trường tàn khốc nơi sinh tồn là tối thượng, nhân tính là điểm yếu chí mạng hay ánh sáng cứu rỗi cuối cùng?', 'RESOLVED', 'Sinh tồn thể xác mà đánh mất đi nhân tính và lòng trắc ẩn thì sự tồn tại đó không còn giá trị nhân văn; chính đạo đức mới là điều phân định con người với loài dã thú.', 'Mất đi nhân tính là mất đi tất cả; sự kiên định bảo vệ phẩm giá trước lằn ranh sinh tử là vinh quang vĩ đại nhất của sự sống.', datetime('now'), datetime('now')),
-- Chủ đề mẹ: Điên loạn và chân lý thực tại · Toàn cầu · nguồn + [Sanity vs. Madness Theme Analysis](https://www.litcharts.com/lit/saint-joan/themes/sanity-vs-madness)
-- PRIMARY: Kẻ Khát Khao Chân Tướng (ACTION) - Dù bị coi là điên loạn vẫn không ngừng truy tìm sự thật nguyên bản · ANTITHESIS: Thể Chế Áp Đặt Chuẩn Mực (VERBAL) - Gán nhãn điên loạn cho bất kỳ tư tưởng dị biệt nào đe dọa trật tự thống trị · CAUTIONARY: Trí Giả Phát Cuồng (OUTCOME) - Bị sự thật vượt tầm hiểu biết nghiền nát tâm trí dẫn đến sụp đổ hoàn toàn · WITNESS: Người Thấu Hiểu Thầm Lặng (IMPLICIT) - Nhìn thấy vẻ đẹp và sự bi thương bên trong thế giới nội tâm của kẻ điên loạn
  ('018f0a00-0001-7000-8000-000000000005', 'THEME_PARENT_EPISTEMOLOGY_AND_MADNESS', NULL, 'Khi thực tại và ảo mộng đảo lộn, tiêu chuẩn nào định nghĩa sự tỉnh táo và bản ngã chân thật của một con người?', 'OPEN', 'Thực tại khách quan thường bị che mờ bởi định kiến và sự áp chế; đôi khi sự điên loạn chỉ là một phản ứng tự vệ tuyệt vọng của tâm trí trước một hiện thực phi nhân tính.', 'Chân lý không phụ thuộc vào số đông công nhận; lòng kiên định với sự chân thật nội tâm là ngọn đèn soi đường qua đêm đen hỗn loạn.', datetime('now'), datetime('now')),
-- Chủ đề mẹ: Đẳng cấp và bình đẳng · Toàn cầu · nguồn + [Social Class and Hierarchy Theme Analysis](https://www.litcharts.com/lit/heart-of-a-dog/themes/social-class-and-hierarchy)
-- PRIMARY: Kẻ Phá Vỡ Gông Cùm Đẳng Cấp (ACTION) - Dùng thực tài và ý chí để thách thức trật tự giai cấp thượng lưu · ANTITHESIS: Tầng Lớp Thống Trị Bảo Thủ (VERBAL) - Duy trì đặc quyền huyết thống và ngăn cản sự thăng tiến của tầng lớp dưới · CAUTIONARY: Kẻ Cơ Hội Giai Cấp (OUTCOME) - Leo lên bằng cách chối bỏ nguồn cội và phản bội những người cùng cảnh ngộ · WITNESS: Tầng Lớp Bình Dân Cần Lao (IMPLICIT) - Lực lượng đông đảo nhất nhưng luôn chịu nhiều thiệt thòi nhất dưới đáy xã hội
  ('018f0a00-0001-7000-8000-000000000006', 'THEME_PARENT_EQUALITY_AND_HIERARCHY', NULL, 'Liệu nỗ lực cá nhân có thể san phẳng bức tường phân biệt giai cấp và định kiến nguồn gốc xã hội?', 'RESOLVED', 'Cấu trúc giai cấp áp bức có thể tồn tại dai dẳng, nhưng ý thức tự giác và sự đoàn kết của những kẻ yếu thế tất yếu sẽ tạo ra những biến chuyển lịch sử không thể đảo ngược.', 'Giá trị con người nằm ở nhân cách và sự cống hiến thực tế, không bao giờ nằm ở hư danh huyết thống hay đặc quyền giai cấp thừa kế.', datetime('now'), datetime('now')),
-- Chủ đề mẹ: Siêu tự sự và quan hệ độc giả · Toàn cầu · nguồn + [Metafiction Explained: How Stories Talk About Themselves](https://literarydevices.net/metafiction-explained-how-stories-talk-about-themselves/)
-- PRIMARY: Độc Giả Tận Hiến (ACTION) - Sống trọn vẹn cùng nhân vật và dùng tình yêu để giải cứu câu chuyện khỏi kết cục định mệnh · ANTITHESIS: Tác Giả / Người Thao Túng Thao Thiết (VERBAL) - Xem nhân vật chỉ là quân cờ phục vụ cho cấu trúc tự sự lạnh lùng · CAUTIONARY: Nhân Vật Mắc Kẹt Kịch Bản (OUTCOME) - Tuyệt vọng vì biết mình chỉ là hình nhân trong trang sách nhưng bất lực không thể tự giải thoát · WITNESS: Cộng Đồng Độc Giả Vô Danh (IMPLICIT) - Hơi thở và sự chú ý tập thể giúp thế giới hư cấu tiếp tục tồn tại sống động
  ('018f0a00-0001-7000-8000-000000000007', 'THEME_PARENT_META_NARRATIVE_AND_TRUTH', NULL, 'Ý nghĩa thực sự của một câu chuyện nằm ở đâu: trong kịch bản định sẵn của tác giả, hay trong sự đồng cảm cứu rỗi của người đọc?', 'RESOLVED', 'Một câu chuyện chỉ thực sự sống khi có người đọc thấu hiểu nó; chính sự kết nối tâm hồn giữa độc giả và nhân vật mới là phép màu phá vỡ sự cô độc của thực tại.', 'Chúng ta đọc truyện để biết mình không đơn độc; mỗi cuộc đời là một câu chuyện đáng được trân trọng và lắng nghe đến dòng cuối cùng.', datetime('now'), datetime('now')),
-- Chủ đề mẹ: Tình yêu, tận hiến và chữa lành · bổ sung khi gộp báo cáo, không có trong ba báo cáo
-- PRIMARY: Kẻ Chữa Lành Bằng Tình Yêu (ACTION) - Dùng lòng kiên trì và bao dung vô điều kiện để cảm hóa tha nhân · ANTITHESIS: Thể Chế Định Kiến / Kẻ Thao Túng Độc Hại (VERBAL) - Coi tình cảm là công cụ trục lợi hoặc ràng buộc ích kỷ · CAUTIONARY: Kẻ Cuồng Yêu Mù Quáng (OUTCOME) - Để tình cảm biến chất thành sự chiếm hữu cực đoan dẫn đến tự hủy diệt · WITNESS: Người Bạn Tri Kỷ Thầm Lặng (IMPLICIT) - Chứng kiến sự trưởng thành và nâng đỡ tinh thần cho nhân vật chính
  ('018f0a00-0001-7000-8000-000000000008', 'THEME_PARENT_LOVE_DEVOTION_AND_HEALING', NULL, 'Liệu tình yêu chân thành và sự thấu cảm vị tha có thể vượt qua định kiến xã hội, chữa lành những tổn thương tâm lý sâu kín nhất của con người?', 'RESOLVED', 'Tình yêu đích thực không phải là sự chiếm hữu hay trói buộc, mà là sự tự do chữa lành, giúp con người hạ cái tôi kiêu hãnh để bao dung và thấu hiểu tha nhân.', 'Tình yêu và lòng trắc ẩn là phương thuốc duy nhất có thể hàn gắn những linh hồn vụn vỡ trước định kiến khắc nghiệt của thế gian.', datetime('now'), datetime('now')),
-- Chủ đề mẹ: Tiếng cười và sự phi lý · bổ sung khi gộp báo cáo, không có trong ba báo cáo
-- PRIMARY: Gã Hề Tỉnh Táo / Anh Hùng Bất Đắc Dĩ (ACTION) - Dùng nụ cười tếu táo để xoa dịu bi kịch và chiến đấu vì điều bình dị · ANTITHESIS: Kẻ Đạo Đức Giả Nghiêm Trang (VERBAL) - Tỏ ra thanh cao, cuồng tín giáo điều nhưng thực chất giả dối · CAUTIONARY: Kẻ Hư Vô Bi Quan (OUTCOME) - Đầu hàng trước sự phi lý của thế giới, rơi vào tuyệt vọng buông xuôi · WITNESS: Khán Giả Thường Dân (IMPLICIT) - Cười vang trước sự ngớ ngẩn của thời cuộc và tìm thấy niềm an ủi
  ('018f0a00-0001-7000-8000-000000000009', 'THEME_PARENT_COMEDY_SATIRE_AND_ABSURDITY', NULL, 'Tiếng cười, sự trào phúng và thái độ lạc quan phi lý có phải là vũ khí tối thượng giúp con người chống lại sự tàn nhẫn và vô nghĩa của cuộc đời?', 'RESOLVED', 'Khi hiện thực quá đỗi khắc nghiệt và giả tạo, tiếng cười châm biếm giúp giải phóng tâm trí, biến bi kịch thành sự thấu hiểu nhẹ nhõm và giúp con người tiếp tục mỉm cười tiến về phía trước.', 'Dù thế giới có ngớ ngẩn và bất công đến đâu, giữ được nụ cười và sự hài hước chính là cách kiêu hãnh nhất để khẳng định phẩm giá sinh tồn.', datetime('now'), datetime('now')),
-- Chủ đề mẹ: Mecha và chiến tranh công nghệ · bổ sung khi gộp báo cáo, không có trong ba báo cáo
-- PRIMARY: Phi Công Đấu Tranh Nhân Tính (ACTION) - Dù mang vết thương lòng vẫn bước vào buồng lái để bảo vệ đồng loại và thấu hiểu đối phương · ANTITHESIS: Cỗ Máy Chiến Tranh Vô Cảm / Thống Chế Quân Phiệt (OUTCOME) - Coi con người là linh kiện tiêu hao phục vụ mưu đồ hủy diệt · CAUTIONARY: Kẻ Trốn Chạy Trong Kén Thép (OUTCOME) - Đồng hóa hoàn toàn với máy móc để chối bỏ nỗi đau làm người · WITNESS: Kỹ Sư / Nhà Điều Hành (IMPLICIT) - Chứng kiến sự mong manh của những thiếu niên mang gánh nặng cứu thế
  ('018f0a00-0001-7000-8000-00000000000a', 'THEME_PARENT_MECHA_AND_TECHNOLOGICAL_WAR', NULL, 'Khi con người hòa mình vào những cỗ máy chiến tranh công nghệ tối thượng, vũ khí là bức tường ngăn cách tâm hồn hay là phương tiện thấu hiểu và bảo vệ sự sống?', 'AMBIGUOUS', 'Công nghệ và cỗ máy chiến tranh khuếch đại cả sức mạnh hủy diệt lẫn nỗi cô độc của con người; chỉ khi vượt qua rào cản phòng vệ tâm lý (A.T. Field) và gắn kết chân thành, con người mới không bị cỗ máy nuốt chửng nhân tính.', 'Sức mạnh công nghệ càng vĩ đại thì trách nhiệm giữ gìn nhân tính càng nặng nề; cỗ máy chiến tranh chỉ có ý nghĩa khi phục vụ cho hòa bình và phẩm giá làm người.', datetime('now'), datetime('now')),
-- Chủ đề mẹ: Giáo dục và trưởng thành · bổ sung khi gộp báo cáo, không có trong ba báo cáo
-- PRIMARY: Người Thầy Khai Phóng / Người Dẫn Đường (ACTION) - Thấu hiểu từng cá nhân, truyền cảm hứng và đánh thức tiềm năng của những học sinh bị gạt ra lề · ANTITHESIS: Thiết Chế Giáo Dục Rập Khuôn (VERBAL) - Ép buộc học sinh vào guồng quay thi cử, thành tích và phân tầng giá trị con người bằng điểm số · CAUTIONARY: Học Sinh Khuất Phục Áp Lực (OUTCOME) - Đánh mất ước mơ và niềm vui sống, rơi vào khủng hoảng hiện sinh vì gánh nặng kỳ vọng · WITNESS: Bạn Đồng Môn (IMPLICIT) - Cùng nhau vượt qua áp lực trưởng thành, chia sẻ ước mơ và hỗ trợ lẫn nhau
  ('018f0a00-0001-7000-8000-00000000000b', 'THEME_PARENT_EDUCATION_AND_YOUTH_GROWTH', NULL, 'Mục đích tối hậu của giáo dục và môi trường học đường là đào tạo những bánh răng phục vụ trật tự xã hội hay khơi dậy tiềm năng, nhân cách và bản ngã độc lập của thế hệ trẻ?', 'RESOLVED', 'Giáo dục chân chính không phải là sự áp đặt hay sàng lọc tàn nhẫn, mà là sự đồng hành, khai mở tiềm năng ẩn giấu và trao cho người học dũng khí tự định nghĩa cuộc đời mình.', 'Thước đo của người thầy vĩ đại không nằm ở điểm số hay danh tiếng, mà ở năng lực thắp sáng niềm tin và trao quyền tự do trưởng thành cho học trò.', datetime('now'), datetime('now')),
-- Chủ đề mẹ: Sống chậm và sinh thái · bổ sung khi gộp báo cáo, không có trong ba báo cáo
-- PRIMARY: Kẻ Trở Về Nguồn Cội / Người Nông Dân Cần Mẫn (ACTION) - Tìm thấy sự thanh thản và tái sinh cảm xúc qua lao động thể chất mộc mạc · ANTITHESIS: Kẻ Theo Đuổi Hào Nhoáng Đô Thị (VERBAL) - Xem thường đời sống giản dị, bị cuốn vào guồng quay vật chất vô tận và kiệt sức · CAUTIONARY: Kẻ Bóc Lột Tự Nhiên (OUTCOME) - Xem tài nguyên đất đai và muôn loài là công cụ trục lợi ngắn hạn, chuốc lấy sự trả thù của sinh thái · WITNESS: Dân Làng Mộc Mạc (IMPLICIT) - Gìn giữ phong tục, sự sẻ chia ấm áp và tinh thần tương thân tương ái
  ('018f0a00-0001-7000-8000-00000000000c', 'THEME_PARENT_SLOW_LIVING_AND_NATURE', NULL, 'Hạnh phúc và sự bình yên đích thực của đời người nằm ở việc không ngừng tranh đoạt danh lợi nơi phồn hoa đô hội hay ở sự an trú trong lao động điền viên mộc mạc và hòa hợp với tự nhiên?', 'RESOLVED', 'Rời xa những vọng tưởng tranh quyền đoạt lợi, con người tìm thấy sự bình yên sâu lắng khi chăm chỉ lao động, tôn trọng nhịp điệu sinh thái và trân trọng sự ấm áp chân tình của những mối liên kết đời thường.', 'Hạnh phúc thực sự không đến từ những điều xa hoa phù phiếm, mà nảy nở từ sự biết ơn những điều giản dị, thuần khiết và chân thành của cuộc sống thường nhật.', datetime('now'), datetime('now'))
ON CONFLICT (code) DO UPDATE SET
  parent_theme_id = EXCLUDED.parent_theme_id, question = EXCLUDED.question,
  resolution_type = EXCLUDED.resolution_type, resolution = EXCLUDED.resolution,
  message = EXCLUDED.message, updated_at = datetime('now');

-- Nhóm 2 · Tác phẩm văn học và manga (báo cáo `subagent_d.md`, 23 dòng)
INSERT INTO themes (id, code, parent_theme_id, question, resolution_type, resolution, message, created_at, updated_at) VALUES
-- Naruto (Nhật Bản · Shonen Manga / Nhẫn giả Huyền huyễn) · chủ đề mẹ: Vòng xoáy thù hận · nguồn + [The Cycle of Hatred in Naruto, Explained](https://www.cbr.com/naruto-cycle-of-hate-explained/)
-- PRIMARY: Uzumaki Naruto (ACTION) · ANTITHESIS: Pain / Nagato & Uchiha Sasuke (VERBAL) · CAUTIONARY: Uchiha Madara & Uchiha Obito (OUTCOME) · WITNESS: Jiraiya (IMPLICIT)
  ('018f0a02-0001-7000-8000-000000000001', 'THEME_NARUTO_CYCLE_OF_HATRED_AND_BOND', '018f0a00-0001-7000-8000-000000000001', 'Làm thế nào để chấm dứt vòng xoáy thù hận truyền kiếp khi nỗi đau cá nhân và sự bất công của hệ thống nhẫn giả luôn tái sinh bạo lực?', 'RESOLVED', 'Hòa bình chân chính không thể đạt được bằng bạo lực răn đe hay ảo mộng trốn chạy, mà phải xây dựng từ lòng dũng cảm gánh chịu nỗi đau thay vì trả thù và sự thấu hiểu chân thành giữa con người.', 'Sự liên kết đồng cảm giữa những tâm hồn từng chịu tổn thương là chiếc cầu duy nhất vượt qua vực thẳm hận thù.', datetime('now'), datetime('now')),
-- Jujutsu Kaisen (Nhật Bản · Dark Fantasy / Siêu nhiên Hiện đại) · chủ đề mẹ: Sinh tồn và nhân tính · nguồn + [Jujutsu Kaisen Thematic Analysis: Order, Disorder, and Hypocrisy](https://www.reddit.com/r/JuJutsuKaisen/comments/o05xa4/jujutsu_kaisen_thematic_analysis_order_disorder/)
-- PRIMARY: Itadori Yuji (ACTION) · ANTITHESIS: Ryomen Sukuna (VERBAL) · CAUTIONARY: Geto Suguru (OUTCOME) · WITNESS: Nanami Kento (OUTCOME)
  ('018f0a02-0002-7000-8000-000000000002', 'THEME_JJK_PROPER_DEATH_AND_EXISTENTIAL_NIHILISM', '018f0a00-0001-7000-8000-000000000004', 'Thế nào là một cái chết đúng nghĩa và giá trị của một sinh mạng nằm ở đâu giữa một thế giới vận hành bằng sự tàn khốc phi lý?', 'AMBIGUOUS', 'Không có cái chết nào là hoàn hảo hay dễ dàng trong vòng xoáy bạo lực; ý nghĩa cuộc đời được định hình qua việc nâng đỡ đồng đội và giữ vững lòng trắc ẩn cho đến hơi thở cuối cùng.', 'Giữa một vũ trụ đầy ác ý và ngẫu nhiên, việc kiên định bảo vệ phẩm giá con người là hành động phản kháng thiêng liêng nhất.', datetime('now'), datetime('now')),
-- Magi: Sinbad no Bouken / The Labyrinth of Magic (Nhật Bản · Fantasy / Sử thi Kỳ ảo) · chủ đề mẹ: Ý chí tự do và định mệnh · nguồn + [Character point about Sinbad and the Rukh destiny in Magi](https://www.reddit.com/r/magi/comments/lrjpiq/character_point_about_sinbad_that_left_a_bad/)
-- PRIMARY: Alibaba Saluja (VERBAL) · ANTITHESIS: Sinbad (ACTION) · CAUTIONARY: David Jehoahaz Abraham (OUTCOME) · WITNESS: Aladdin (IMPLICIT)
  ('018f0a02-0003-7000-8000-000000000003', 'THEME_MAGI_DESTINY_RUKH_AND_COLLECTIVE_FREE_WILL', '018f0a00-0001-7000-8000-000000000002', 'Liệu con người có nên giao phó vận mệnh cho một nhà độc tài khai sáng toàn năng để đạt được thiên đường không xung đột, hay giữ quyền tự do dù phải đối mặt với sai lầm?', 'RESOLVED', 'Thế giới thuộc về tất cả mọi người; việc tước đoạt ý chí tự do nhân danh trật tự là hình thức nô dịch nguy hiểm nhất. Con người phải tự chịu trách nhiệm và cùng nhau giải quyết mâu thuẫn bằng trí tuệ tập thể.', 'Tự do đích thực bao gồm cả quyền được vấp ngã và tự mình đứng dậy; không một cá nhân vĩ đại nào có quyền tước đoạt tương lai của đồng loại.', datetime('now'), datetime('now')),
-- Tensei Shitara Slime Datta Ken (Nhật Bản · Isekai / Quốc gia Xây dựng) · chủ đề mẹ: Đẳng cấp và bình đẳng · nguồn + [Nation Building and the Role of Leadership in That Time I Got Reincarnated as a Slime](https://iopn.library.illinois.edu/journals/jams/article/download/922/927/4125)
-- PRIMARY: Rimuru Tempest (ACTION) · ANTITHESIS: Clayman & Yuuki Kagurazaka (VERBAL) · CAUTIONARY: Vua Edmaris của Falmuth (OUTCOME) · WITNESS: Gazel Dwargon (VERBAL)
  ('018f0a02-0004-7000-8000-000000000004', 'THEME_TENSURA_COEXISTENCE_AND_PRAGMATIC_UTOPIA', '018f0a00-0001-7000-8000-000000000006', 'Làm sao để các giống loài dị biệt có thể chung sống hòa bình trong một thế giới mà định kiến và lợi ích sinh tồn luôn thúc đẩy thanh trừng?', 'RESOLVED', 'Sự chung sống hòa bình chỉ bền vững khi đi đôi với sức mạnh bảo vệ kiên quyết kết hợp cùng sự phồn vinh kinh tế và chính sách ngoại giao tôn trọng phẩm giá các bên.', 'Lý tưởng nhân đạo thuần khiết sẽ sụp đổ nếu thiếu sức mạnh bảo vệ; hòa bình thực sự là thành quả của sự bao dung đi kèm năng lực tự vệ tuyệt đối.', datetime('now'), datetime('now')),
-- Black Clover (Nhật Bản · Shonen Fantasy / Ma pháp Chiến đấu) · chủ đề mẹ: Đẳng cấp và bình đẳng · nguồn + [BLACK CLOVER VOL 1: Exploring Themes of Class and Meritocracy](https://test.mwp.com/edition/RSYEqc0AD023/BlackCloverVol1)
-- PRIMARY: Asta (ACTION) · ANTITHESIS: Giới Quý tộc Vương quốc Clover (VERBAL) · CAUTIONARY: Patry / Licht giả (OUTCOME) · WITNESS: Yami Sukehiro (IMPLICIT)
  ('018f0a02-0005-7000-8000-000000000005', 'THEME_BLACK_CLOVER_MERITOCRACY_VS_CLASSISM', '018f0a00-0001-7000-8000-000000000006', 'Liệu ý chí kiên định và sự nỗ lực của kẻ ở đáy cùng xã hội có thể phá vỡ hệ thống giai cấp cố hữu lấy huyết thống và tư bản bẩm sinh làm thước đo?', 'RESOLVED', 'Hành động cụ thể và sự kiên trì không lùi bước có thể làm thay đổi nhận thức của cả một vương quốc, chứng minh rằng giá trị của một người do việc họ làm định đoạt.', 'Nghịch cảnh không phải xiềng xích định đoạt số phận; sự bình đẳng thực sự nảy nở khi người yếu nhất chứng minh được phẩm giá và tạo cảm hứng đoàn kết.', datetime('now'), datetime('now')),
-- One Piece (Nhật Bản · Phiêu lưu Hải tặc / Sử thi Thế giới) · chủ đề mẹ: Ý chí tự do và định mệnh · nguồn + [One Piece: Towards a Figural Semiopraxis of Freedom](https://www.academia.edu/143499918/One_Piece_Towards_a_figural_semiopraxis_of_freedom)
-- PRIMARY: Monkey D. Luffy (ACTION) · ANTITHESIS: Imu & Ngũ Lão Tinh (VERBAL) · CAUTIONARY: Sakazuki / Akainu (OUTCOME) · WITNESS: Nico Robin & Silvers Rayleigh (IMPLICIT)
  ('018f0a02-0006-7000-8000-000000000006', 'THEME_ONE_PIECE_FREEDOM_VS_SYSTEMIC_TYRANNY', '018f0a00-0001-7000-8000-000000000002', 'Tự do đích thực là gì trong một thế giới bị thống trị bởi một chế độ độc tài ngụy trang dưới ngọn cờ Chính nghĩa tuyệt đối?', 'RESOLVED', 'Tự do không phải là quyền lực cai trị kẻ khác, mà là quyền được sống trọn vẹn với ước mơ của mình và dũng cảm đập tan xiềng xích áp bức để chia sẻ tự do đó cho tha nhân.', 'Ước mơ của con người, dòng chảy thời đại và ý chí kế thừa là những điều không bao giờ có thể bị dập tắt chừng nào con người còn khao khát tự do.', datetime('now'), datetime('now')),
-- Attack on Titan / Shingeki no Kyojin (Nhật Bản · Dark Fantasy / Bi kịch Quân sự) · chủ đề mẹ: Vòng xoáy thù hận · nguồn + [The Philosophy of Freedom and Social Contract Theory in Attack on Titan](https://publications.lycoming.edu/mid-atlantic-humanities-review/volume-3/6-serena-martin.pdf)
-- PRIMARY: Eren Jaeger (OUTCOME) · ANTITHESIS: Zeke Jaeger (VERBAL) · CAUTIONARY: Armin Arlert & Liên Minh (VERBAL) · WITNESS: Reiner Braun (IMPLICIT)
  ('018f0a02-0007-7000-8000-000000000007', 'THEME_AOT_FREEDOM_DETERMINISM_AND_GENOCIDE', '018f0a00-0001-7000-8000-000000000001', 'Để giành lấy tự do trước mối đe dọa hủy diệt từ thế giới, con người có quyền biến mình thành quái vật đồ sát nhân loại hay không?', 'AMBIGUOUS', 'Vòng lặp bạo lực là một phần bi kịch dai dẳng của lịch sử; việc dùng diệt chủng để kiếm tìm tự do chỉ biến cá nhân thành nô lệ tội nghiệp của chính nỗi sợ hãi và thù hận.', 'Tự do không thể xây đắp trên đống tro tàn của sự diệt chủng; chừng nào con người chưa dám bước ra khỏi khu rừng thù hận, bi kịch sẽ mãi tái diễn.', datetime('now'), datetime('now')),
-- Fullmetal Alchemist (Nhật Bản · Steampunk Fantasy / Triết học Giả kim) · chủ đề mẹ: Tha hóa quyền lực · nguồn + [Fullmetal Alchemist and the Exaltation of Humanity](https://www.thefandomentals.com/fullmetal-alchemist-humanity/)
-- PRIMARY: Edward Elric (ACTION) · ANTITHESIS: Cha / Father (OUTCOME) · CAUTIONARY: Shou Tucker (OUTCOME) · WITNESS: Roy Mustang & Riza Hawkeye (VERBAL)
  ('018f0a02-0008-7000-8000-000000000008', 'THEME_FMA_EQUIVALENT_EXCHANGE_AND_HUMAN_HUBRIS', '018f0a00-0001-7000-8000-000000000003', 'Liệu quy luật Trao đổi đồng giá lạnh lùng có giải quyết được mọi nỗi đau nhân sinh, hay sự cứu rỗi thực sự nằm ở sự cho đi không vụ lợi?', 'RESOLVED', 'Quy luật trao đổi cơ học không thể đo đếm được tình yêu và lòng trắc ẩn; đỉnh cao của trí tuệ con người là sẵn sàng từ bỏ quyền năng vô biên để trở về làm người phàm yêu thương đồng loại.', 'Con người nhỏ bé trước vũ trụ, nhưng sự vĩ đại đích thực nằm ở khả năng sửa chữa sai lầm, tình thương không toan tính và sự gắn kết giữa những trái tim bất toàn.', datetime('now'), datetime('now')),
-- Hunter x Hunter (Nhật Bản · Dark Fantasy / Shonen Giải cấu trúc) · chủ đề mẹ: Sinh tồn và nhân tính · nguồn + [Humanity Explored in Chimera Ant Arc](https://www.scribd.com/document/487255213/HxH)
-- PRIMARY: Gon Freecss (ACTION) · ANTITHESIS: Meruem (OUTCOME) · CAUTIONARY: Isaac Netero (OUTCOME) · WITNESS: Killua Zoldyck (IMPLICIT)
  ('018f0a02-0009-7000-8000-000000000009', 'THEME_HXH_HUMAN_MALICE_VS_MONSTROUS_EVOLUTION', '018f0a00-0001-7000-8000-000000000004', 'Ai mới là quái vật thực sự khi quái vật ngoại lai học được tình yêu vị tha, còn con người lại bộc lộ sự tàn nhẫn vô hạn để duy trì chuỗi thức ăn sinh tồn?', 'AMBIGUOUS', 'Ranh giới thiện ác có tính tương đối và mong manh; bản chất sinh tồn của nhân loại chứa đựng sự độc ác tột cùng được ngụy trang dưới danh nghĩa lý trí và nghĩa vụ giống loài.', 'Con người không phải loài sinh vật thánh thiện nhất mà là loài nguy hiểm nhất bởi dã tâm sinh tồn tàn khốc ẩn sâu dưới lớp vỏ bọc văn minh.', datetime('now'), datetime('now')),
-- Vinland Saga (Nhật Bản · Sử thi Lịch sử / Viking Bi tráng) · chủ đề mẹ: Vòng xoáy thù hận · nguồn + [The Philosophy of Vinland Saga](https://tiagovf.medium.com/the-philosophy-of-vinland-saga-22cfc0450c24)
-- PRIMARY: Thorfinn Karlsefni (ACTION) · ANTITHESIS: Vua Canute (VERBAL) · CAUTIONARY: Askeladd (OUTCOME) · WITNESS: Thors Snorresson (OUTCOME)
  ('018f0a02-0010-7000-8000-000000000010', 'THEME_VINLAND_TRUE_WARRIOR_AND_RADICAL_PACIFISM', '018f0a00-0001-7000-8000-000000000001', 'Liệu lý tưởng hòa bình tuyệt đối (''Tôi không có kẻ thù nào cả'') có thể tồn tại và cứu rỗi con người trong một thế giới được xây dựng trên máu, gươm giáo và cướp bóc?', 'OPEN', 'Trở thành chiến binh thực thụ là con đường chuộc tội gian nan không hồi kết; từ chối bạo lực đòi hỏi lòng quả cảm lớn hơn việc cầm vũ khí tước đoạt sinh mạng người khác.', 'Ngươi không có kẻ thù nào cả; sức mạnh vĩ đại nhất của con người nằm ở năng lực nhẫn nại, yêu thương và vun đắp sự sống thay vì gieo rắc sự hủy diệt.', datetime('now'), datetime('now')),
-- Phàm Nhân Tu Tiên (Trung Quốc · Tiên hiệp Cổ điển / Phàm nhân lưu) · chủ đề mẹ: Sinh tồn và nhân tính · nguồn + [Hàn Lập Tu Tiên Niên Biểu - Phân Tích Thực Dụng Sinh Tồn](https://phamnhan.fandom.com/vi/wiki/H%C3%A0n_L%E1%BA%ADp_tu_ti%C3%AAn_ni%C3%AAn_bi%E1%BB%83u)
-- PRIMARY: Hàn Lập (ACTION) · ANTITHESIS: Mặc Đại Phu / Mặc Cư Nhân (OUTCOME) · CAUTIONARY: Lệ Phi Vũ (OUTCOME) · WITNESS: Nam Cung Uyển (IMPLICIT)
  ('018f0a02-0011-7000-8000-000000000011', 'THEME_RMJI_SURVIVAL_PRAGMATISM_AND_DAO_PERSISTENCE', '018f0a00-0001-7000-8000-000000000004', 'Làm thế nào một phàm nhân tư chất bình thường có thể sinh tồn và nghịch thiên cải mệnh trong một thế giới cá lớn nuốt cá bé mà không đánh mất điểm mấu chốt của nhân tính?', 'RESOLVED', 'Tu tiên hay sinh tồn đòi hỏi sự cẩn trọng tột cùng, lý trí thực tế không màng hư danh, nhưng sự kiên định với giới hạn đạo đức nền tảng mới là thứ giữ cho đạo tâm không bị ma hóa.', 'Không có con đường tắt nào dẫn đến đại đạo; sự nhẫn nại phi thường, tỉnh táo trước cám dỗ và lòng kiên trì với bản ngã là vũ khí mạnh nhất của người phàm.', datetime('now'), datetime('now')),
-- Thiếu Niên Ca Hành (Trung Quốc · Tân Võ hiệp / Thiếu niên Du hiệp) · chủ đề mẹ: Ý chí tự do và định mệnh · nguồn + [Thiếu Niên Ca Hành: Tinh Thần Nghĩa Khí Và Bất Khuất Trước Vương Quyền](https://hananinareweb.wordpress.com/2019/07/09/thieu-nien-ca-hanh-chu-moc-nam/)
-- PRIMARY: Tiêu Sắt / Tiêu Sở Hà (ACTION) · ANTITHESIS: Tiêu Vũ / Xích Vương (OUTCOME) · CAUTIONARY: Minh Đức Đế & Lạc Thanh Dương (VERBAL) · WITNESS: Lôi Vô Kiệt & Vô Tâm (IMPLICIT)
  ('018f0a02-0012-7000-8000-000000000012', 'THEME_SPOY_YOUTH_CHIVALRY_AND_DESTINY_DEFIANCE', '018f0a00-0001-7000-8000-000000000002', 'Khí phách thiếu niên và nghĩa khí giang hồ có thể giữ được sự thuần khiết trước những âm mưu tranh quyền đoạt vị tàn khốc của vương triều phong kiến?', 'RESOLVED', 'Quyền lực ngai vàng không thể mua được tấm chân tình; danh vị thế tục hoàn toàn lu mờ trước một bầu rượu tri kỷ và tự do tự tại của tuổi trẻ dám sống hết mình.', 'Thiếu niên vĩnh viễn không cúi đầu trước số phận bất công; ý nghĩa cao nhất của cuộc đời là sự dấn thân vì bằng hữu và giữ trọn trái tim xích tử.', datetime('now'), datetime('now')),
-- Khói Lửa Nhân Gian Của Tôi / Một Tòa Thành Đang Chờ Anh (Trung Quốc · Hiện đại Đô thị / Tâm lý Xã hội) · chủ đề mẹ: Ý chí tự do và định mệnh · nguồn + [Một Tòa Thành Đang Chờ Anh - Khói Lửa Nhân Gian Của Tôi: Khát Vọng Tự Chủ Nhân Cách](https://dep.com.vn/ket-cuc-tham-hai-cua-khoi-lua-nhan-gian-cua-toi-khi-di-nguoc-nguyen-tac/)
-- PRIMARY: Tống Diệm (ACTION) · ANTITHESIS: Phó Văn Anh (VERBAL) · CAUTIONARY: Mạnh Yến Thần (OUTCOME) · WITNESS: Hứa Thấm (ACTION)
  ('018f0a02-0013-7000-8000-000000000013', 'THEME_FIREWORKS_INDIVIDUAL_AUTONOMY_VS_FAMILY_CONTROL', '018f0a00-0001-7000-8000-000000000002', 'Một cá nhân có thể thoát khỏi chiếc lồng kính quyền quý và sự thao túng tâm lý gia tộc để tìm kiếm tự do nhân phẩm và tình yêu đích thực?', 'RESOLVED', 'Tự do và hạnh phúc không thể xây dựng trên sự lệ thuộc vật chất và phục tùng tinh thần; phẩm giá thực sự chỉ nở hoa khi con người dám tự lập và sống đúng với cảm xúc chân thành.', 'Hạnh phúc không nằm trong sự giàu sang tù túng mà được đúc kết từ khói lửa bình dị của nhân gian và dũng khí chịu trách nhiệm về chính sinh mệnh của mình.', datetime('now'), datetime('now')),
-- Quỷ Bí Chi Chủ / Lord of the Mysteries (Trung Quốc · Cthulhu Mythos / Steampunk Ma pháp) · chủ đề mẹ: Điên loạn và chân lý thực tại · nguồn + [Master the Beyonder Pathways: Acting and Madness in Lord of the Mysteries](https://world.taobao.com/lang/en-us/shopping-guide/2023949543007059968.htm)
-- PRIMARY: Klein Moretti / The Fool (ACTION) · ANTITHESIS: Amon / Kẻ Độc Thần (VERBAL) · CAUTIONARY: Ince Zangwill & George III (OUTCOME) · WITNESS: Audrey Hall (IMPLICIT)
  ('018f0a02-0014-7000-8000-000000000014', 'THEME_LOTM_HUMANITY_VS_DIVINITY_AND_MADNESS', '018f0a00-0001-7000-8000-000000000005', 'Khi chạm tới đỉnh cao quyền năng Thần thánh và đối mặt với sự ô nhiễm vô tận của vũ trụ, làm thế nào để bản ngã con người không bị điên loạn nuốt chửng?', 'RESOLVED', 'Thần tính mang lại quyền năng nhưng nhân tính mới là ngọn hải đăng cứu rỗi duy nhất; sức mạnh đỉnh cao không phải để thống trị mà là để gánh vác trách nhiệm bảo vệ kẻ yếu.', 'Chúng ta là những kẻ đáng thương luôn phải chiến đấu chống lại sự điên loạn; lòng trắc ẩn và những gắn kết tình người bình dị là chiếc neo giữ ta là chính mình.', datetime('now'), datetime('now')),
-- Đạo Quỷ Dị Tiên (Trung Quốc · Cthulhu Tu tiên / Hiện sinh Tâm thần) · chủ đề mẹ: Điên loạn và chân lý thực tại · nguồn + [Đạo Quỷ Dị Tiên Và Khủng Hoảng Hiện Sinh Của Lý Hỏa Vượng](https://www.facebook.com/groups/731382050771436/posts/1987119538531008/)
-- PRIMARY: Lý Hỏa Vượng (ACTION) · ANTITHESIS: Lão Đan Dương Tử (OUTCOME) · CAUTIONARY: Chư Cát Uyên (OUTCOME) · WITNESS: Bạch Linh Miểu & Bác sĩ Dịch Đông Lai (IMPLICIT)
  ('018f0a02-0015-7000-8000-000000000015', 'THEME_DAO_GUI_YI_XIAN_SANITY_TRUTH_AND_EXISTENTIAL_PAIN', '018f0a00-0001-7000-8000-000000000005', 'Khi ranh giới giữa bệnh viện tâm thần và thế giới quỷ dị hoàn toàn sụp đổ, điều gì định nghĩa chân tướng thực tại và cứu rỗi giá trị tồn tại của con người?', 'AMBIGUOUS', 'Không có tiêu chuẩn khách quan tuyệt đối nào để phân định chân - giả giữa những thế giới hỗn loạn; ý nghĩa sinh tồn được quyết định bởi tình yêu và sự bảo vệ chân thành dành cho tha nhân.', 'Ngay cả khi cả vũ trụ bị điên loạn bóp méo, lòng trắc ẩn và sự hy sinh vì những người ta trân quý vẫn là sự thật thiêng liêng duy nhất không bao giờ bị vấy bẩn.', datetime('now'), datetime('now')),
-- Toàn Chức Cao Thủ (Trung Quốc · Võng du eSports / Đô thị Thể thao Điện tử) · chủ đề mẹ: Đẳng cấp và bình đẳng · nguồn + [For The Sake Of Glory: The King's Avatar Review](https://kvasir369.wordpress.com/2020/12/27/for-the-sake-of-glory-the-kings-avatar-review/)
-- PRIMARY: Diệp Tu / Diệp Thu (ACTION) · ANTITHESIS: Đào Hiên & Tôn Tường (VERBAL) · CAUTIONARY: Lưu Hạo (OUTCOME) · WITNESS: Trần Quả & Tô Mộc Tranh (IMPLICIT)
  ('018f0a02-0016-7000-8000-000000000016', 'THEME_QUAN_ZHI_PURE_PASSION_VS_COMMERCIALISM', '018f0a00-0001-7000-8000-000000000006', 'Liệu tình yêu thuần túy với đam mê có thể chiến thắng sự đào thải nghiệt ngã của tuổi tác và áp lực thương mại hóa kim tiền trong thể thao chuyên nghiệp?', 'RESOLVED', 'Sự đào thải chỉ đánh bại những kẻ tự mãn; khi tình yêu xuất phát từ đam mê nguyên bản không vụ lợi, con người có thể biến thất bại thành điểm tựa tái sinh đỉnh cao rực rỡ.', 'Nếu bạn thực sự yêu thích nó, hãy xem nó là vinh quang chứ không phải sự khoe khoang; nhiệt huyết chân chính và tinh thần đồng đội không bao giờ già đi trước thời gian.', datetime('now'), datetime('now')),
-- Khánh Dư Niên (Trung Quốc · Xuyên không Lịch sử / Quyền mưu Trọng sinh) · chủ đề mẹ: Tha hóa quyền lực · nguồn + [Ye Qingmei and Ideological Conflict in Joy of Life](https://baike.baidu.com/en/item/Ye%20Qingmei/16820)
-- PRIMARY: Phạm Nhàn (ACTION) · ANTITHESIS: Khánh Đế (VERBAL) · CAUTIONARY: Diệp Khinh Mai (OUTCOME) · WITNESS: Trần Bình Bình (VERBAL)
  ('018f0a02-0017-7000-8000-000000000017', 'THEME_QING_YU_NIAN_EGALITARIAN_IDEALISM_VS_FEUDAL_ABSOLUTISM', '018f0a00-0001-7000-8000-000000000003', 'Liệu tư tưởng hiện đại về bình đẳng và nhân quyền có thể bám rễ và lay chuyển được thể chế phong kiến chuyên chế thâm căn cố đế?', 'OPEN', 'Một cá nhân không thể một sớm một chiều thay đổi bánh xe lịch sử phong kiến ngàn năm; nhưng hạt mầm nhân quyền và phẩm giá con người khi đã gieo xuống sẽ không bao giờ tàn lụi.', 'Sinh mệnh con người sinh ra không phân biệt quý tiện; sống trọn vẹn những năm tháng đời người là dám đứng lên bảo vệ công lý cho những kẻ yếu thế.', datetime('now'), datetime('now')),
-- Tru Tiên (Trung Quốc · Tiên hiệp Cổ điển / Thần ma Bi tráng) · chủ đề mẹ: Tha hóa quyền lực · nguồn + [Review Tru Tiên - Khảo Sát Thần Ma Và Sự Giả Tạo Chính Tà](https://www.facebook.com/Huyenaoquan2710/posts/review-tru-ti%C3%AAn-c%E1%BA%A3m-nh%E1%BA%ADn-v%E1%BB%81-si%C3%AAu-ph%E1%BA%A9m-ti%C3%AAn-hi%E1%BB%87p-m%E1%BB%9F-%C4%91%E1%BA%A7u-cho-m%E1%BB%99t-th%E1%BB%9Di-%C4%91%E1%BA%A1i-m%E1%BB%9Bi-c%E1%BB%A7a-/217395223494480/)
-- PRIMARY: Trương Tiểu Phàm / Quỷ Lệ (ACTION) · ANTITHESIS: Đạo Huyền Chân Nhân (OUTCOME) · CAUTIONARY: Phổ Trí Thần Tăng (OUTCOME) · WITNESS: Bích Dao & Lục Tuyết Kỳ (ACTION)
  ('018f0a02-0018-7000-8000-000000000018', 'THEME_ZHU_XIAN_ORTHODOXY_HYPOCRISY_AND_HEAVEN_INDIFFERENCE', '018f0a00-0001-7000-8000-000000000003', 'Thiên địa bất nhân dĩ vạn vật vi sô cẩu — đâu là ranh giới thực sự giữa Chính và Tà khi danh môn chính phái lại đầy rẫy sự ngụy quân tử và tàn độc?', 'RESOLVED', 'Chính và Tà không nằm ở danh xưng môn phái hay thần thông tu luyện, mà nằm ở tâm địa và hành động đối đãi với sinh mệnh; trời đất vô tình nhưng lòng người hữu ái.', 'Đừng để giáo điều và định kiến che mờ lương tri; sự hy sinh chân thành vì tình yêu và lòng trắc ẩn vượt lên trên mọi ranh giới phân biệt thế tục.', datetime('now'), datetime('now')),
-- Omniscient Reader's Viewpoint (Hàn Quốc · K-Webnovel / Siêu hư cấu Metafiction) · chủ đề mẹ: Siêu tự sự và quan hệ độc giả · nguồn + [Postmodern Narrative Logic in Sing Shong's Omniscient Reader's Viewpoint](https://www.academia.edu/171694359/Postmodern_narrative_logic_in_Sing_Shong_s_Omniscient_reader_s_viewpoint)
-- PRIMARY: Kim Dokja (ACTION) · ANTITHESIS: Các Tinh Tọa & Dokkaebi (VERBAL) · CAUTIONARY: Giấc Mơ Cổ Đại / The Ancient Dream (OUTCOME) · WITNESS: Han Sooyoung & Yoo Joonghyuk (ACTION)
  ('018f0a02-0019-7000-8000-000000000019', 'THEME_ORV_READER_SALVATION_AND_META_DETERMINISM', '018f0a00-0001-7000-8000-000000000007', 'Liệu một Độc giả tầm thường có thể cứu lấy nhân vật và thế giới khỏi số phận bi kịch định sẵn bằng sự đồng cảm và thấu hiểu sâu sắc trang sách cuộc đời?', 'RESOLVED', 'Bằng sự dấn thân và tình yêu vô điều kiện dành cho câu chuyện, người đọc có thể trở thành đấng sáng tạo đồng hành, viết lại cái kết bi kịch thành khúc ca hy vọng ấm áp.', 'Câu chuyện này tồn tại là vì bạn, người đã đọc nó đến dòng chữ cuối cùng; sự đồng cảm chân thành giữa các tâm hồn có thể cứu vớt cả một vũ trụ khỏi sự cô đơn.', datetime('now'), datetime('now')),
-- Solo Leveling (Hàn Quốc · Urban Fantasy / Thợ săn Đột kích) · chủ đề mẹ: Sinh tồn và nhân tính · nguồn + [Solo Leveling Analysis: Themes of Power, Isolation, and Transformation](https://www.studocu.com/en-ca/document/centennial-college/english-12/solo-leveling-analysis-themes-of-power-isolation-and-transformation/138547223)
-- PRIMARY: Sung Jin-Woo (ACTION) · ANTITHESIS: Các Quân Vương Ngoại Giới / Antares (VERBAL) · CAUTIONARY: Hwang Dong-Su (OUTCOME) · WITNESS: Sung Il-Hwan (OUTCOME)
  ('018f0a02-0020-7000-8000-000000000020', 'THEME_SOLO_LEVELING_SOLITUDE_OF_POWER_AND_SACRIFICE', '018f0a00-0001-7000-8000-000000000004', 'Khi gánh vác sức mạnh tối thượng để cứu vớt nhân loại trước sự hủy diệt của Thần linh, cái giá của sự cô độc và tha hóa nhân tính có đáng để đánh đổi?', 'RESOLVED', 'Quyền năng tối thượng không phải để hưởng thụ danh vọng mà là gánh nặng trách nhiệm; sự hy sinh thầm lặng trong bóng tối để bảo vệ ánh sáng bình yên cho người khác là thước đo của anh hùng đích thực.', 'Đỉnh cao của quyền lực là sự cô độc, nhưng tình yêu gia đình và ý chí bảo vệ đồng loại biến sự cô độc ấy thành hành động hy sinh cao cả nhất.', datetime('now'), datetime('now')),
-- The Lord of the Rings (Anh Quốc · High Fantasy / Sử thi Kỳ ảo Cổ điển) · chủ đề mẹ: Tha hóa quyền lực · nguồn + [The Quest for Pity and Mercy in Tolkien's Middle Earth](https://pillars.taylor.edu/cgi/viewcontent.cgi?article=1116&context=inklings_forever)
-- PRIMARY: Frodo Baggins & Samwise Gamgee (ACTION) · ANTITHESIS: Chúa tể Sauron (OUTCOME) · CAUTIONARY: Boromir & Saruman (OUTCOME) · WITNESS: Gandalf Áo Trắng (VERBAL)
  ('018f0a02-0021-7000-8000-000000000021', 'THEME_LOTR_CORRUPTION_OF_POWER_AND_MERCY', '018f0a00-0001-7000-8000-000000000003', 'Làm thế nào cái thiện nhỏ bé và yếu ớt có thể chống lại sức cám dỗ ghê gớm của Quyền lực tuyệt đối khi không ai có thể hoàn toàn miễn nhiễm trước nó?', 'RESOLVED', 'Quyền lực tuyệt đối không thể bị đánh bại bằng bạo lực tương đương mà chỉ có thể bị hóa giải bằng lòng thương xót, sự khiêm nhường và đức hy sinh quên mình của những người nhỏ bé nhất.', 'Ngay cả người nhỏ bé nhất cũng có thể thay đổi tiến trình của tương lai; lòng từ bi và đức khiêm nhường là sức mạnh mạnh hơn mọi vũ khí hủy diệt.', datetime('now'), datetime('now')),
-- Dune / Xứ Cát (Hoa Kỳ · Khoa học Viễn tưởng / Sử thi Chính trị Sinh thái) · chủ đề mẹ: Tha hóa quyền lực · nguồn + [Dune: A Leadership Critique](https://researchcenter.trt.net.tr/publications/discussion-paper/dune-a-leadership-critique/)
-- PRIMARY: Paul Atreides / Muad'Dib (OUTCOME) · ANTITHESIS: Hội Bene Gesserit & Hoàng đế Shaddam IV (VERBAL) · CAUTIONARY: Stilgar & Người Fremen (OUTCOME) · WITNESS: Chani & Công chúa Irulan (IMPLICIT)
  ('018f0a02-0022-7000-8000-000000000022', 'THEME_DUNE_DANGER_OF_CHARISMATIC_LEADERS', '018f0a00-0001-7000-8000-000000000003', 'Liệu niềm tin mù quáng vào một Đấng Cứu Thế siêu phàm có giải thoát nhân loại, hay nó là cái bẫy chết người đẩy xã hội vào thảm họa thánh chiến diệt chủng?', 'RESOLVED', 'Các nhà lãnh đạo lôi cuốn và thần quyền tôn giáo luôn là công cụ thao túng chính trị nguy hiểm; trao trọn lý trí cho một cá nhân cứu tinh tất yếu dẫn tới sự nô dịch và hủy diệt tập thể.', 'Hãy cảnh giác trước các vị cứu thế; sự an toàn của nhân loại nằm ở tư duy phản biện độc lập và sự kiểm soát quyền lực, không bao giờ nằm ở việc tôn sùng cá nhân mù quáng.', datetime('now'), datetime('now')),
-- 1984 (Anh Quốc · Dystopia / Phản địa đàng Toàn trị) · chủ đề mẹ: Điên loạn và chân lý thực tại · nguồn + [Surveillance and Control in George Orwell's 1984](https://ijels.com/upload_document/issue_files/30IJELS-101202541-Surveillance.pdf)
-- PRIMARY: Winston Smith (OUTCOME) · ANTITHESIS: O'Brien & Đảng / Big Brother (VERBAL) · CAUTIONARY: Syme (OUTCOME) · WITNESS: Julia (ACTION)
  ('018f0a02-0023-7000-8000-000000000023', 'THEME_1984_TOTALITARIANISM_DOUBLETHINK_AND_TRUTH', '018f0a00-0001-7000-8000-000000000005', 'Liệu nhân tính, tình yêu và chân lý khách quan có thể tồn tại khi một thể chế toàn trị có khả năng tái định nghĩa ngôn ngữ và bóp méo ký ức con người?', 'RESOLVED', 'Trước sự kiểm soát tâm lý và bạo lực toàn trị tuyệt đối, ý chí phản kháng đơn độc của cá nhân sẽ bị nghiền nát; tự do chỉ tồn tại khi con người bảo vệ được quyền nói lên sự thật khách quan.', 'Tự do là quyền được nói hai cộng hai bằng bốn. Nếu sự thật bị bóp méo và ngôn ngữ bị tha hóa, nhân loại sẽ rơi vào đêm trường nô dịch vĩnh cửu.', datetime('now'), datetime('now'))
ON CONFLICT (code) DO UPDATE SET
  parent_theme_id = EXCLUDED.parent_theme_id, question = EXCLUDED.question,
  resolution_type = EXCLUDED.resolution_type, resolution = EXCLUDED.resolution,
  message = EXCLUDED.message, updated_at = datetime('now');

-- Nhóm 3 · Game và tác phẩm mở rộng (báo cáo `subagent_e.md`, 20 dòng)
INSERT INTO themes (id, code, parent_theme_id, question, resolution_type, resolution, message, created_at, updated_at) VALUES
-- Elden Ring / Dark Souls (FromSoftware / Nhật Bản · Action RPG / Dark Fantasy) · chủ đề mẹ: Ý chí tự do và định mệnh · nguồn + [Existentialist Philosophical Analysis of Dark Souls and Elden Ring](https://thegemsbok.com/art-reviews-and-articles/dark-souls-from-software-existentialism-philosophy/)
-- PRIMARY: Ranni the Witch / Ashen One (ACTION) · ANTITHESIS: Lord Gwyn / Queen Marika / Radagon (OUTCOME) · CAUTIONARY: Goldmask / The Corvian Settler (VERBAL) · WITNESS: Melina / Firekeeper (IMPLICIT)
  ('018f0a02-0001-7111-8a01-000000000001', 'THEME_CYCLE_OF_DECAY_AND_DIVINE_ORDER', '018f0a00-0001-7000-8000-000000000002', 'Khi một trật tự vàng son đã suy tàn và biến thành ngục tù giam hãm sinh linh, con người nên tiếp tục hy sinh để níu kéo ảo ảnh quá khứ hay can đảm bước vào bóng tối bất định để tìm kiếm sự đổi mới?', 'RESOLVED', 'Chấm dứt sự duy trì gượng ép của trật tự cũ; dập tắt ngọn lửa tàn và để bóng tối cùng các vì sao tiếp quản. Thế giới cần được giải thoát khỏi sự thao túng của các thực thể thần thánh siêu nhiên để tự định hình vận mệnh.', 'Sự trì trệ dưới danh nghĩa trật tự vĩnh hằng chỉ sinh ra đau khổ và quái thai. Can đảm đón nhận cái chết của cái cũ là điều kiện duy nhất để sự sống thực sự được tái sinh.', datetime('now'), datetime('now')),
-- God of War: Norse Saga (Santa Monica Studio / Mỹ · Action-Adventure / Mythological Fantasy) · chủ đề mẹ: Vòng xoáy thù hận · nguồn + [The Evolution of Masculinity and Fatherhood in God of War](https://digitalcommons.kennesaw.edu/cgi/viewcontent.cgi?article=1325&context=kjur)
-- PRIMARY: Kratos (VERBAL) · ANTITHESIS: Odin (OUTCOME) · CAUTIONARY: Freya / Thor (ACTION) · WITNESS: Mimir (IMPLICIT)
  ('018f0a02-0002-7111-8a01-000000000002', 'THEME_FATHERHOOD_AND_CYCLE_OF_VIOLENCE', '018f0a00-0001-7000-8000-000000000001', 'Liệu một người mang quá khứ tội lỗi và bản năng bạo lực có thể thay đổi để nuôi dạy thế hệ sau trở nên tốt đẹp hơn, chấm dứt chu kỳ thù hận truyền đời?', 'RESOLVED', 'Kratos lựa chọn không bước vào vết xe đổ của sự hủy diệt, hạ vũ khí tha thứ và dốc lòng bảo bọc, hướng thiện cho con trai. Số phận không phải là bản án bất di dịch mà được tạo nên bởi chính những lựa chọn đạo đức của từng khoảnh khắc.', 'Quá khứ không định nghĩa bản chất con người; lòng can đảm chân chính của người làm cha mẹ là phá vỡ chuỗi thương tổn của thế hệ trước để con cái được sống với tự do và lòng nhân ái.', datetime('now'), datetime('now')),
-- The Last of Us Part I & II (Naughty Dog / Mỹ · Post-Apocalyptic / Action-Adventure) · chủ đề mẹ: Vòng xoáy thù hận · nguồn + [Thematic Analysis of The Cycle of Vengeance and Morality in The Last of Us](https://medium.com/@FegoBeGaming/thematic-analysis-of-the-cycle-of-vengeance-violence-in-the-last-of-us-part-2-678dfdf05cb7)
-- PRIMARY: Ellie (ACTION) · ANTITHESIS: Joel / Abby (OUTCOME) · CAUTIONARY: Tommy (ACTION) · WITNESS: Lev (IMPLICIT)
  ('018f0a02-0003-7111-8a01-000000000003', 'THEME_LOVE_MORALITY_AND_CYCLE_OF_REVENGE', '018f0a00-0001-7000-8000-000000000001', 'Khi tình yêu mãnh liệt biến thành sự thù hận tột cùng sau mất mát, liệu con người có thể buông bỏ sự báo thù trước khi nó phá hủy hoàn toàn nhân tính và những gì còn sót lại?', 'RESOLVED', 'Ellie dừng lại ở ngưỡng cửa của việc trở thành con quái vật như kẻ cô căm ghét. Việc tha thứ cho Abby thực chất là sự tha thứ muộn màng dành cho Joel và chuộc lỗi với chính sự bất lực trong tâm hồn mình.', 'Tình yêu vị kỷ và sự báo thù là hai mặt của cùng một lưỡi dao sắc nhọn; chỉ khi dũng cảm buông bỏ chấp niệm thù hận, con người mới có thể thực sự bắt đầu tiến trình chữa lành những vết thương tâm hồn.', datetime('now'), datetime('now')),
-- Red Dead Redemption 2 (Rockstar Games / Mỹ · Open-World Western / Drama) · chủ đề mẹ: Sinh tồn và nhân tính · nguồn + [Red Dead Redemption 2: The Ruins of an Outlaw Era and Late Morality](https://www.pointnthink.fr/en/red-dead-redemption-2-the-ruins-of-a-world/)
-- PRIMARY: Arthur Morgan (ACTION) · ANTITHESIS: Dutch van der Linde (VERBAL) · CAUTIONARY: Micah Bell (OUTCOME) · WITNESS: John Marston (IMPLICIT)
  ('018f0a02-0004-7111-8a01-000000000004', 'THEME_OUTLAW_DECAY_AND_LATE_REDEMPTION', '018f0a00-0001-7000-8000-000000000004', 'Trong một thời đại đang băng hoại và khi cái chết đã gõ cửa, liệu một kẻ đã trót sống cả đời trong tội lỗi có còn kịp tìm lại danh dự và sự cứu rỗi cho linh hồn?', 'RESOLVED', 'Sự cứu chuộc không xóa bỏ được những tội lỗi đã qua trong quá khứ, nhưng nó mang lại ý nghĩa cho cái chết thông qua việc hy sinh bản thân để bảo vệ sự sống và tương lai cho người khác.', 'Dù quá khứ có tăm tối đến đâu, con người luôn có quyền lựa chọn làm điều đúng đắn ở ngã rẽ cuối cùng của cuộc đời; sự cứu chuộc nằm ở việc biết đặt sinh mệnh người khác lên trên cái tôi của chính mình.', datetime('now'), datetime('now')),
-- Cyberpunk 2077 (CD Projekt Red / Ba Lan · Sci-Fi Action RPG / Cyberpunk) · chủ đề mẹ: Đẳng cấp và bình đẳng · nguồn + [Cyberpunk 2077: Humanity, Identity, and Hyper-Corporatism in Night City](https://medium.com/@VariousThemes/a-subjective-review-and-analysis-of-cyberpunk-2077-d906d086889d)
-- PRIMARY: V (ACTION) · ANTITHESIS: Saburo Arasaka (OUTCOME) · CAUTIONARY: Johnny Silverhand (VERBAL) · WITNESS: Judy Alvarez / Viktor Vector (IMPLICIT)
  ('018f0a02-0005-7111-8a01-000000000005', 'THEME_CYBERPUNK_HUMANITY_VS_HYPER_CAPITALISM', '018f0a00-0001-7000-8000-000000000006', 'Trong một thế giới tư bản công nghệ nơi mọi giá trị nhân tính bị biến thành món hàng, con người sống thế nào để giữ được bản sắc và linh hồn độc lập trước cái chết cận kề?', 'AMBIGUOUS', 'Không có chiến thắng tuyệt đối trước cỗ máy tập đoàn khổng lồ. Con người chỉ có thể tìm thấy sự cứu rỗi qua những mối liên kết chân thật, tình yêu thương và quyền tự quyết định cái chết hoặc tự do của bản thân.', 'Ý nghĩa cuộc sống không nằm ở ánh hào quang giả tạo của danh vọng huyền thoại tại Night City, mà ở khả năng giữ vững nhân tính và kết nối với những người mình yêu thương giữa thời đại tha hóa.', datetime('now'), datetime('now')),
-- NieR: Automata (Square Enix & PlatinumGames / Nhật Bản · Action RPG / Philosophical Sci-Fi) · chủ đề mẹ: Siêu tự sự và quan hệ độc giả · nguồn + [NieR: Automata: Meaning and Existential Despair in a Post-Human World](https://kristoforusas.medium.com/nier-automata-meaning-existential-despair-fe1f457407c1)
-- PRIMARY: Pod 042 & Pod 153 / Ending E (ACTION) · ANTITHESIS: YoRHa Command / The Logic Virus (OUTCOME) · CAUTIONARY: 9S (OUTCOME) · WITNESS: Pascal (VERBAL)
  ('018f0a02-0006-7111-8a01-000000000006', 'THEME_EXISTENTIAL_PURPOSE_IN_GODLESS_WORLD', '018f0a00-0001-7000-8000-000000000007', 'Khi mục đích sống vốn được áp đặt bởi đấng sáng tạo sụp đổ hoàn toàn, sinh mệnh có thể tự tìm thấy ý nghĩa và giá trị tồn tại trong một vũ trụ vô nghĩa hay không?', 'RESOLVED', 'Ý nghĩa không được ban phát từ bên ngoài mà được sinh ra từ chính những mối liên kết, tình yêu thương và sự tương trợ giữa các sinh linh. Cùng nhau vượt qua nghịch cảnh và giúp đỡ người khác là hành động tối thượng kiến tạo ý nghĩa.', 'Một thế giới không có thần linh không phải là bản án của sự tuyệt vọng, mà là mảnh đất tự do để mỗi cá nhân tự viết nên mục đích sống thông qua lòng trắc ẩn và sự kết nối giữa các tâm hồn.', datetime('now'), datetime('now')),
-- Black Myth: Wukong (Game Science / Trung Quốc · Action RPG / Mythological Dark Fantasy) · chủ đề mẹ: Ý chí tự do và định mệnh · nguồn + [The Symbolism of Freedom and Restraint: Golden Headband in Black Myth Wukong](https://timebladeguild.com/golden-headband-in-black-myth-wukong/)
-- PRIMARY: Tôn Ngộ Không / Thiên Mệnh Nhân (ACTION) · ANTITHESIS: Linh Cát Bồ Tát / Thiên Đình & Tây Phương (VERBAL) · CAUTIONARY: Hoàng Bào Quái / Chư thần đọa lạc (OUTCOME) · WITNESS: Nhị Lang Chân Quân (IMPLICIT)
  ('018f0a02-0007-7111-8a01-000000000007', 'THEME_FREEDOM_VS_DIVINE_SUBJUGATION', '018f0a00-0001-7000-8000-000000000002', 'Tu thành chính quả và có được sự an toàn trong khuôn khổ giáo điều có đáng giá hơn việc giữ vững ý chí tự do và bản ngã bất khuất của bản thân?', 'RESOLVED', 'Không có quả vị hay sự bảo hộ thần thánh nào có thể đổi lấy sự tự do của linh hồn. Phải dũng cảm phá vỡ Vòng Kim Cô của định kiến và quyền lực để làm chủ vận mệnh của chính mình.', 'Sự vĩ đại không đến từ chiếc vương miện hay sự công nhận của các thế lực bề trên, mà tỏa sáng từ ý chí bất khuất dám chiến đấu chống lại sự nô dịch tinh thần.', datetime('now'), datetime('now')),
-- The Witcher 3: Wild Hunt (CD Projekt Red / Ba Lan · Dark Fantasy RPG) · chủ đề mẹ: Sinh tồn và nhân tính · nguồn + [Story Structure and Morality: The Dilemma of The Lesser Evil in The Witcher](https://www.how-to-write-a-book-now.com/the-lesser-evil.html)
-- PRIMARY: Geralt of Rivia (ACTION) · ANTITHESIS: Radovid V / Eredin (OUTCOME) · CAUTIONARY: Nam tước Đỏ (ACTION) · WITNESS: Ciri (IMPLICIT)
  ('018f0a02-0008-7111-8a01-000000000008', 'THEME_LESSER_EVIL_AND_MORAL_NEUTRALITY', '018f0a00-0001-7000-8000-000000000004', 'Trong một thế giới xám xịt nơi không có lựa chọn nào hoàn toàn trong sạch, liệu con người có thể giữ vị thế trung lập hay buộc phải chọn "cái ác nhỏ hơn" để bảo vệ điều thiện?', 'AMBIGUOUS', 'Sự trung lập tuyệt đối là ảo tưởng hèn nhát. Con người không thể trốn tránh việc lựa chọn, và đôi khi phải chấp nhận chịu ô danh khi chọn phương án ít tổn hại nhất cho những người vô tội.', 'Lương tâm không cho phép chúng ta ngoảnh mặt làm ngơ trước đau khổ; dũng cảm gánh vác trách nhiệm của một lựa chọn không hoàn hảo chính là biểu hiện cao nhất của tính người.', datetime('now'), datetime('now')),
-- Honkai: Star Rail & Genshin Impact (HoYoverse / Trung Quốc · Space Fantasy / Open-World ARPG) · chủ đề mẹ: Ý chí tự do và định mệnh · nguồn + [Data Bank Lore and Aeons Philosophy: The Paths of Cosmos](https://hoyodex.miraheze.org/wiki/Data_Bank_%28Honkai:_Star_Rail)
-- PRIMARY: The Trailblazer / Akivili / Dainsleif (ACTION) · ANTITHESIS: Nanook the Destruction / Sunday & Order (VERBAL) · CAUTIONARY: Cocolia, Mother of Deception (OUTCOME) · WITNESS: Acheron (IMPLICIT)
  ('018f0a02-0009-7111-8a01-000000000009', 'THEME_HUMAN_AGENCY_VS_COSMIC_DETERMINISM', '018f0a00-0001-7000-8000-000000000002', 'Trước sự nhỏ bé tuyệt đối của loài người trước định mệnh vũ trụ và các thực thể thần thánh vô biên, ý chí và sự vùng vẫy của phàm nhân có thực sự mang lại ý nghĩa?', 'RESOLVED', 'Dù cái kết của vũ trụ có thể là hư vô hay diệt vong, giá trị của sinh mệnh nằm trọn vẹn trong hành trình khai phá và những dấu chân kết nối yêu thương giữa các cá thể phàm trần.', 'Chúng ta sinh ra không phải để tuân theo kịch bản của thần linh; vẻ đẹp vĩ đại nhất của sự sống là lòng dũng cảm dấn bước vào cõi vô định để tự tay viết nên lịch sử của giống loài.', datetime('now'), datetime('now')),
-- Final Fantasy VII (Square Enix / Nhật Bản · JRPG / Eco-Dystopian Fantasy) · chủ đề mẹ: Sống chậm và sinh thái · nguồn + [Final Fantasy VII: Central Themes of Environmental Degradation and Identity Fragmentation](https://stuartgough.medium.com/final-fantasy-vii-a-deep-dive-into-its-central-themes-through-critical-theory-117b4b99c920)
-- PRIMARY: Cloud Strife & Tifa Lockhart (ACTION) · ANTITHESIS: Sephiroth (OUTCOME) · CAUTIONARY: Shinra Executives / Hojo (ACTION) · WITNESS: Aerith Gainsborough (IMPLICIT)
  ('018f0a02-0010-7111-8a01-000000001001', 'THEME_ECOLOGICAL_SACRIFICE_AND_IDENTITY_CRISIS', '018f0a00-0001-7000-8000-00000000000c', 'Liệu con người có thể chiến thắng lòng tham khai thác tàn bạo để sống hòa hợp với sinh mệnh của hành tinh, và làm sao một cá nhân có thể tìm lại bản ngã đích thực sau những chấn thương tâm lý nặng nề?', 'RESOLVED', 'Hành tinh chỉ có thể được cứu rỗi khi con người từ bỏ việc bóc lột thiên nhiên và học cách tôn trọng cội nguồn sự sống. Mỗi người chỉ có thể tìm thấy sức mạnh chân chính khi dám đối diện và chấp nhận sự yếu đuối của bản thân.', 'Thiên nhiên là nguồn cội của mọi sinh linh chứ không phải mỏ tài nguyên vô tận; sự trưởng thành đích thực bắt đầu từ giây phút can đảm tháo bỏ chiếc mặt nạ kiêu ngạo để sống thật với chính mình.', datetime('now'), datetime('now')),
-- Thế Giới Hoàn Mỹ (Thần Đông / Trung Quốc · Tiên Hiệp / Huyền Huyễn) · chủ đề mẹ: Sinh tồn và nhân tính · nguồn + [Phân tích Thế Giới Hoàn Mỹ: Hoang Thiên Đế và Nỗi Cô Độc Độc Đoán Vạn Cổ](https://thuvienanime.net/the-gioi-hoan-my/)
-- PRIMARY: Thạch Hạo (ACTION) · ANTITHESIS: Thi Hài Tiên Đế / Cổ Thần Cung Cấp Hắc Ám (OUTCOME) · CAUTIONARY: Thạch Nghị / Trọng Đồng Giả (ACTION) · WITNESS: Liễu Thần (IMPLICIT)
  ('018f0a02-0011-7111-8a01-000000001002', 'THEME_SUPREME_LONELINESS_FOR_SALVATION', '018f0a00-0001-7000-8000-000000000004', 'Cái giá thực sự của sức mạnh tối thượng là gì, và liệu một cá nhân có thể chịu đựng được sự cô độc tuyệt đối để gánh vác sinh mệnh của cả một kỷ nguyên?', 'RESOLVED', 'Đỉnh cao của sức mạnh không phải là vinh quang hay sự hưởng lạc, mà là trách nhiệm và sự hy sinh vô điều kiện. Hoang Thiên Đế đã dùng sự cô đơn vĩnh hằng của mình để đổi lấy bình yên cho muôn đời sau.', 'Sức mạnh càng to lớn thì gánh nặng hy sinh càng tột cùng; người bảo vệ chân chính là người dám đứng một mình trong bão tố bóng tối để những người phía sau được sống trong ánh bình minh.', datetime('now'), datetime('now')),
-- Cổ Chân Nhân (Reverend Insanity · Cổ Chân Nhân / Trung Quốc · Tiên Hiệp Ma Đạo / Triết lý Thực Dụng) · chủ đề mẹ: Sinh tồn và nhân tính · nguồn + [Reverend Insanity: The Dark Philosophy and Utilitarianism of Fang Yuan](https://nischaldawadi.com.np/books/reverend_insanity/)
-- PRIMARY: Phương Nguyên (ACTION) · ANTITHESIS: Thiên Đình / Túc Mệnh Cổ (OUTCOME) · CAUTIONARY: Phương Trịnh / Bạch Ngưng Băng (ACTION) · WITNESS: Thái Ách Thần Bí / Lục Thất Tuyệt (IMPLICIT)
  ('018f0a02-0012-7111-8a01-000000001003', 'THEME_ABSOLUTE_UTILITARIANISM_AND_PERSEVERANCE', '018f0a00-0001-7000-8000-000000000004', 'Khi một cá nhân cởi bỏ hoàn toàn mọi rào cản đạo đức thế tục để theo đuổi lý tưởng tối thượng, ranh giới giữa một kẻ kiên trì phi thường và một con quỷ tàn độc nằm ở đâu?', 'AMBIGUOUS', 'Tác phẩm không phán xét đạo đức theo lăng kính nhị nguyên tốt - xấu. Thế giới là một đấu trường sinh tồn khắc nghiệt nơi kẻ chiến thắng là kẻ có ý chí bất khuất và khả năng tận dụng mọi quy luật khách quan.', 'Ý chí kiên định theo đuổi con đường đã chọn mà không hối tiếc là một phẩm chất đáng sợ; tuy nhiên, việc vứt bỏ toàn bộ tính người sẽ biến cuộc đời thành một hành trình cô độc và lạnh lẽo đến tận cùng.', datetime('now'), datetime('now')),
-- Nhất Niệm Vĩnh Hằng (A Will Eternal · Nhĩ Căn / Trung Quốc · Tiên Hiệp Hài Hước & Trưởng Thành) · chủ đề mẹ: Sinh tồn và nhân tính · nguồn + [Tóm tắt và Đánh giá Nhất Niệm Vĩnh Hằng: Từ Sợ Chết Đến Trách Nhiệm Vĩnh Cửu](https://thuvienanime.net/nhat-niem-vinh-hang/)
-- PRIMARY: Bạch Tiểu Thuần (ACTION) · ANTITHESIS: Thông Thiên Đạo Nhân (OUTCOME) · CAUTIONARY: Đỗ Lăng Phi (VERBAL) · WITNESS: Lý Thanh Hậu (IMPLICIT)
  ('018f0a02-0013-7111-8a01-000000001004', 'THEME_FEAR_OF_DEATH_TRANSFORMED_INTO_RESPONSIBILITY', '018f0a00-0001-7000-8000-000000000004', 'Liệu bản năng sinh tồn và nỗi sợ cái chết có thể trở thành cội nguồn của lòng dũng cảm phi thường khi nó được gắn liền với tình yêu thương và trách nhiệm?', 'RESOLVED', 'Nỗi sợ hãi cái chết không phải là sự hèn nhát nếu nó thúc đẩy con người trân quý sự sống. Khi nỗi sợ chuyển hóa thành khát vọng bảo vệ người khác, nó sẽ biến kẻ nhút nhát nhất thành anh hùng vĩ đại nhất.', 'Sống lâu không bằng sống có ý nghĩa; sự bất tử đích thực không nằm ở tuổi thọ sinh học vô tận mà ở những giá trị và tình cảm thiêng liêng mà ta dám dùng cả sinh mệnh để bảo vệ.', datetime('now'), datetime('now')),
-- Đại Phụng Đả Canh Nhân (Mại Báo Tiểu Lang Quân / Trung Quốc · Tiên Hiệp / Trinh Thám Lịch Sử) · chủ đề mẹ: Tha hóa quyền lực · nguồn + [Đại Phụng Đả Canh Nhân: Vì Dân Thỉnh Mệnh và Cuộc Chiến Hoàng Quyền](https://thuvienanime.net/dai-phung-da-canh-nhan/)
-- PRIMARY: Hứa Thất An (ACTION) · ANTITHESIS: Nguyên Cảnh Đế / Trấn Bắc Vương (OUTCOME) · CAUTIONARY: Ngụy Uyên (ACTION) · WITNESS: Trịnh Hưng Hoài (VERBAL)
  ('018f0a02-0014-7111-8a01-000000001005', 'THEME_PLEBEIAN_JUSTICE_AGAINST_IMPERIAL_CORRUPTION', '018f0a00-0001-7000-8000-000000000003', 'Khi luật pháp và hoàng quyền trở thành công cụ áp bức của kẻ thống trị, một kẻ tiểu nhân hạ đẳng có nên đánh đổi tính mạng để đòi lại công lý cho bách tính vô danh?', 'RESOLVED', 'Công lý và đạo nghĩa nhân dân đứng trên thiên mệnh hoàng quyền. Kẻ cầm quyền phản bội bách tính thì ắt sẽ bị lòng dân và sức mạnh của chính nghĩa phế truất.', 'Giá trị cao quý nhất của người cầm vũ khí và nắm quyền lực không phải là phục vụ đế vương, mà là bảo vệ những số phận yếu thế và dám lên tiếng trước sự bất công của thời đại.', datetime('now'), datetime('now')),
-- Berserk (Kentaro Miura / Nhật Bản · Dark Fantasy Manga) · chủ đề mẹ: Ý chí tự do và định mệnh · nguồn + [Berserk: Ambition, Fate, and the Unbreakable Human Condition](https://medium.com/@renas_/berserk-a-tale-of-ambition-fate-and-the-human-condition-ae67583a9f1e)
-- PRIMARY: Guts (ACTION) · ANTITHESIS: Griffith (OUTCOME) · CAUTIONARY: Skull Knight (VERBAL) · WITNESS: Casca (IMPLICIT)
  ('018f0a02-0015-7111-8a01-000000001006', 'THEME_FREE_WILL_STRUGGLE_AGAINST_CAUSALITY', '018f0a00-0001-7000-8000-000000000002', 'Khi toàn bộ vũ trụ và dòng chảy số phận tiền định đã tuyên án cái chết và sự bất lực lên một con người, việc tiếp tục vùng vẫy đấu tranh có mang lại ý nghĩa gì?', 'RESOLVED', 'Chính hành động vật lộn và không chịu đầu hàng đã mang lại ý nghĩa cho sự tồn tại. Nhân quả có thể định hình hoàn cảnh, nhưng ý chí con người quyết định thái độ và phẩm giá khi đối mặt với nghịch cảnh.', 'Dù định mệnh có nghiệt ngã và tăm tối đến đâu, chừng nào con người còn tiếp tục vung kiếm và tiến bước, ngọn lửa của ý chí tự do sẽ không bao giờ bị dập tắt.', datetime('now'), datetime('now')),
-- Death Note (Tsugumi Ohba & Takeshi Obata / Nhật Bản · Psychological Thriller) · chủ đề mẹ: Tha hóa quyền lực · nguồn + [Death Note: The Moral Dilemma of Justice and Absolute Power](https://hypercritic.org/collection/death-note-manga-review-analysis-2003)
-- PRIMARY: L (VERBAL) · ANTITHESIS: Light Yagami (OUTCOME) · CAUTIONARY: Misa Amane / Teru Mikami (ACTION) · WITNESS: Soichiro Yagami (IMPLICIT)
  ('018f0a02-0016-7111-8a01-000000001007', 'THEME_VIGILANTE_JUSTICE_VS_RULE_OF_LAW', '018f0a00-0001-7000-8000-000000000003', 'Liệu mục tiêu loại bỏ hoàn toàn tội phạm có thể biện minh cho việc một cá nhân đứng trên pháp luật để nắm quyền phán xét sự sống và cái chết của đồng loại?', 'RESOLVED', 'Quyền năng phán xét tuyệt đối không thể tạo nên một thế giới tốt đẹp hơn mà chỉ sinh ra chế độ độc tài dựa trên sợ hãi. Một kẻ tự cho mình quyền giết chóc nhân danh công lý sớm muộn cũng sẽ trở thành con quỷ dữ tồi tệ nhất.', 'Công lý chân chính không được xây dựng trên nỗi sợ và sự độc tài cá nhân; sự tôn trọng quy trình pháp lý và sinh mạng con người là nền tảng tối thượng của văn minh.', datetime('now'), datetime('now')),
-- Monster (Naoki Urasawa / Nhật Bản · Psychological Suspense / Seinen Manga) · chủ đề mẹ: Đẳng cấp và bình đẳng · nguồn + [Exploring the Philosophical Depths of Monster: Are Human Lives Truly Equal?](https://magica.com/youtube-summarizer/exploring-the-philosophical-depths-of-monster-are-human-lives-truly-equal-KYYVMWDVXA4)
-- PRIMARY: Dr. Kenzo Tenma (ACTION) · ANTITHESIS: Johan Liebert (VERBAL) · CAUTIONARY: Thanh tra Heinrich Lunge (OUTCOME) · WITNESS: Nina Fortner (IMPLICIT)
  ('018f0a02-0017-7111-8a01-000000001008', 'THEME_EQUALITY_OF_LIFE_AND_NATURE_OF_EVIL', '018f0a00-0001-7000-8000-000000000006', 'Khi một hành động nhân đạo cứu sống một sinh mạng lại vô tình thả ra một con quái vật reo rắc thảm họa, liệu nguyên lý "Mọi sinh mạng đều bình đẳng" có còn đứng vững?', 'RESOLVED', 'Sinh mạng của mọi con người đều có giá trị bình đẳng bất khả xâm phạm. Cái ác sinh ra từ sự tước đoạt danh tính, tình yêu thương và những thí nghiệm tàn bạo của con người; và nó chỉ có thể bị hóa giải bằng lòng trắc ẩn kiên định chứ không phải bạo lực trả đũa.', 'Mọi đứa trẻ sinh ra đều có quyền được có một cái tên và được yêu thương; việc giữ vững niềm tin vào lòng nhân ái giữa bóng tối tuyệt vọng là chiến thắng lớn nhất của phẩm giá con người.', datetime('now'), datetime('now')),
-- Re:Zero - Starting Life in Another World (Tappei Nagatsuki / Nhật Bản · Dark Fantasy Isekai) · chủ đề mẹ: Tình yêu, tận hiến và chữa lành · nguồn + [Re:Zero and Self-Acceptance: Overcoming the Toxic Hero Complex](https://rockymountaincentral.substack.com/p/rezero-and-self-acceptance)
-- PRIMARY: Natsuki Subaru (ACTION) · ANTITHESIS: Echidna (VERBAL) · CAUTIONARY: Petelgeuse Romanee-Conti (OUTCOME) · WITNESS: Rem / Emilia (IMPLICIT)
  ('018f0a02-0018-7111-8a01-000000001009', 'THEME_OVERCOMING_HERO_COMPLEX_AND_SELF_HATRED', '018f0a00-0001-7000-8000-000000000008', 'Liệu việc liên tục hy sinh bản thân và chịu đựng cái chết có phải là con đường cao thượng để trở thành anh hùng, hay đó chỉ là sự trốn chạy sự bất lực và căm ghét chính mình?', 'RESOLVED', 'Hy sinh thân mình mà không trân trọng bản thân chỉ là một dạng bạo lực tinh thần trá hình. Một người chỉ có thể thực sự cứu giúp và yêu thương người khác khi họ biết chấp nhận những thiếu sót và trân quý sinh mệnh của chính mình.', 'Bạn không cần phải trở thành một đấng toàn năng hay chết đi nghìn lần để xứng đáng được yêu thương; bắt đầu lại từ con số không với lòng chân thành và sự tôn trọng bản thân chính là sự dũng cảm vĩ đại nhất.', datetime('now'), datetime('now')),
-- Steins;Gate (5pb. & Nitroplus / Chiyomaru Shikura / Nhật Bản · Sci-Fi Visual Novel / Thriller) · chủ đề mẹ: Ý chí tự do và định mệnh · nguồn + [Anatomy of Steins;Gate: The Subjective Loneliness of the Observer](https://note.com/akira_fumizawa/n/n7e2a2cdb08da?hl=en)
-- PRIMARY: Okabe Rintaro (ACTION) · ANTITHESIS: SERN / Ủy ban 300 (OUTCOME) · CAUTIONARY: Suzuha Amane (ACTION) · WITNESS: Makise Kurisu (IMPLICIT)
  ('018f0a02-0019-7111-8a01-000000001010', 'THEME_BURDEN_OF_THE_OBSERVER_AND_TIME_CAUSALITY', '018f0a00-0001-7000-8000-000000000002', 'Cái giá của việc can thiệp vào dòng thời gian để thay đổi số mệnh là gì, và một cá nhân có thể chịu đựng được bao nhiêu nỗi cô đơn khi là người duy nhất ghi nhớ những thế giới đã bị xóa sổ?', 'RESOLVED', 'Không thể tùy tiện thay đổi quá khứ mà không trả giá đắt. Thay vì thao túng sự kiện để ép buộc kết quả, giải pháp là tôn trọng dòng chảy nhân quả và khéo léo thay đổi góc nhìn quan sát để tạo nên tương lai mới mà không làm sụp đổ hiện tại.', 'Mỗi khoảnh khắc trôi qua và từng mối liên kết của hiện tại đều vô giá; hãy sống trọn vẹn với hiện tại thay vì ám ảnh việc quay ngược thời gian để sửa chữa những tiếc nuối.', datetime('now'), datetime('now')),
-- Brave New World (Aldous Huxley / Vương Quốc Anh · Dystopian Classic Novel) · chủ đề mẹ: Ý chí tự do và định mệnh · nguồn + [Brave New World at 75: The Cost of Programmed Comfort vs The Right to Suffer](https://www.thenewatlantis.com/publications/brave-new-world-at-75)
-- PRIMARY: John the Savage (VERBAL) · ANTITHESIS: Mustapha Mond (VERBAL) · CAUTIONARY: Bernard Marx / Helmholtz Watson (ACTION) · WITNESS: Lenina Crowne (OUTCOME)
  ('018f0a02-0020-7111-8a01-000000001011', 'THEME_MANUFACTURED_HAPPINESS_VS_AUTHENTIC_FREEDOM', '018f0a00-0001-7000-8000-000000000002', 'Liệu một xã hội không có chiến tranh và đau khổ nhưng đổi lại bằng việc triệt tiêu tự do, nghệ thuật và cảm xúc chân thật có phải là thiên đường đáng sống?', 'OPEN', 'Tác phẩm không đưa ra một cái kết màu hồng dễ dãi. John the Savage tự sát vì không thể dung hòa giữa bản ngã tự nhiên và sự ô uế của thế giới mới; Mustapha Mond vẫn tiếp tục vận hành hệ thống. Câu hỏi về sự đánh đổi giữa tiện nghi thỏa mãn và tự do bản ngã được để ngỏ cho toàn thể nhân loại suy ngẫm.', 'Hạnh phúc đích thực không thể tách rời khỏi khả năng cảm nhận nỗi đau và tự do lựa chọn; một cuộc sống tiện nghi nhưng bị tước đoạt linh hồn và quyền suy nghĩ độc lập chỉ là sự giam cầm êm ái.', datetime('now'), datetime('now'))
ON CONFLICT (code) DO UPDATE SET
  parent_theme_id = EXCLUDED.parent_theme_id, question = EXCLUDED.question,
  resolution_type = EXCLUDED.resolution_type, resolution = EXCLUDED.resolution,
  message = EXCLUDED.message, updated_at = datetime('now');

-- Nhóm 4 · Lãng mạn, hài hước, mecha, học viện, đời thường (báo cáo `subagent_f.md`, 30 dòng)
INSERT INTO themes (id, code, parent_theme_id, question, resolution_type, resolution, message, created_at, updated_at) VALUES
-- Kiêu Hãnh và Định Kiến (Jane Austen · Anh Quốc · Lãng mạn) · chủ đề mẹ: Tình yêu, tận hiến và chữa lành · nguồn + [SparkNotes: Pride and Prejudice Themes & Literary Analysis](https://www.sparknotes.com/lit/pride/themes/)
-- PRIMARY: Elizabeth Bennet (ACTION & VERBAL) · ANTITHESIS: Lady Catherine de Bourgh (VERBAL) · CAUTIONARY: Lydia Bennet & George Wickham (OUTCOME) · WITNESS: Jane Bennet & Charles Bingley (IMPLICIT)
  ('018f0a03-0001-7000-8000-000000000001', 'THEME_PRIDE_PREJUDICE_SELF_AWARENESS', '018f0a00-0001-7000-8000-000000000008', 'Liệu con người có thể vượt qua định kiến giai cấp và sự kiêu hãnh cá nhân để nhận diện phẩm giá và tình yêu chân chính?', 'RESOLVED', 'Elizabeth và Darcy đều tự nguyện từ bỏ sự kiêu hãnh và định kiến, thấu hiểu phẩm cách nội tâm của nhau để tiến tới hôn nhân bình đẳng về mặt tâm hồn.', 'Tình yêu đích thực đòi hỏi sự khiêm nhường để tự nhìn nhận khiếm khuyết của bản thân và dũng khí vượt qua rào cản quy ước xã hội.', datetime('now'), datetime('now')),
-- Ma Đạo Tổ Sư (Mặc Hương Đồng Khứu · Trung Quốc · Lãng mạn / Tiên hiệp Đam mỹ) · chủ đề mẹ: Tha hóa quyền lực · nguồn + [TV Tropes: Analysis / Grandmaster of Demonic Cultivation](https://tvtropes.org/pmwiki/pmwiki.php/Analysis/GrandmasterOfDemonicCultivationMoDaoZuShi)
-- PRIMARY: Ngụy Vô Tiện & Lam Vong Cơ (ACTION) · ANTITHESIS: Kim Quang Dao & Tiên Môn Bách Gia (ACTION & VERBAL) · CAUTIONARY: Giang Trừng (OUTCOME) · WITNESS: Lam Tư Truy (IMPLICIT)
  ('018f0a03-0002-7000-8000-000000000002', 'THEME_MDZS_MORALITY_VS_HYPOCRISY', '018f0a00-0001-7000-8000-000000000003', 'Giữa đạo đức ước lệ của đám đông và lương tri thực sự, kẻ tu đạo nên thuận theo thế tục hay kiên định với chính đạo trong lòng?', 'RESOLVED', 'Ngụy Vô Tiện tái sinh và cùng Lam Vong Cơ phơi bày sự giả tạo của Tiên môn bách gia, chọn rời xa quyền lực thế tục để tiêu dao thiên hạ cùng nhau.', 'Chính nghĩa thực sự không nằm ở danh xưng hay số đông phán xét, mà ở bản lĩnh bảo vệ kẻ yếu và thủy chung với tri kỷ bất chấp định kiến thế gian.', datetime('now'), datetime('now')),
-- Thiên Quan Tứ Phúc (Mặc Hương Đồng Khứu · Trung Quốc · Lãng mạn / Tiên hiệp Đam mỹ) · chủ đề mẹ: Tình yêu, tận hiến và chữa lành · nguồn + [SuperSummary: Heaven Official's Blessing Character & Thematic Analysis](https://www.supersummary.com/heaven-officials-blessing/major-character-analysis/)
-- PRIMARY: Tạ Liên & Hoa Thành (ACTION & IMPLICIT) · ANTITHESIS: Quân Ngô / Bạch Vô Tướng (ACTION & VERBAL) · CAUTIONARY: Thích Dung & Sư Vô Độ (OUTCOME) · WITNESS: Mộ Tình & Phong Tín (IMPLICIT)
  ('018f0a03-0003-7000-8000-000000000003', 'THEME_TGCF_DEVOTION_AND_COMPASSION', '018f0a00-0001-7000-8000-000000000008', 'Liệu lòng vị tha và tình yêu đơn thuần có thể đứng vững trước sự suy đồi của thần quyền và sự hiểm ác của lòng người?', 'RESOLVED', 'Tạ Liên phá vỡ ách thống trị thao túng của Quân Ngô bằng lòng từ bi bất biến; sự tận tụy 800 năm của Hoa Thành giúp cả hai tìm thấy hạnh phúc vĩnh hằng.', 'Dù cả thế giới sụp đổ hay chối bỏ, chỉ cần còn một tín đồ kiên định và một trái tim giữ vững thiện niệm, con người sẽ không bao giờ gục ngã trước bóng tối.', datetime('now'), datetime('now')),
-- NANA (Ai Yazawa · Nhật Bản · Lãng mạn / Josei) · chủ đề mẹ: Tình yêu, tận hiến và chữa lành · nguồn + [Sabukaru Online: Nana and Vivienne Westwood - A Manga's Ode to Punk Style & Identity](https://sabukaru.online/articles/nana-and-vivienne-westwood-a-mangas-ode-to-punk-style)
-- PRIMARY: Nana Osaki (ACTION) · ANTITHESIS: Nana Komatsu / Hachi (VERBAL & ACTION) · CAUTIONARY: Ren Honjo (OUTCOME) · WITNESS: Yasushi Takagi / Yasu (IMPLICIT)
  ('018f0a03-0004-7000-8000-000000000004', 'THEME_NANA_INDEPENDENCE_VS_CO_DEPENDENCE', '018f0a00-0001-7000-8000-000000000008', 'Người phụ nữ hiện đại làm thế nào để đạt được độc lập tự chủ mà không đánh mất sự kết nối tình cảm và rơi vào bi kịch cô độc?', 'OPEN', 'Mỗi Nana phải tự gánh chịu hệ quả từ lựa chọn của mình: Hachi học cách tự lập khi làm mẹ, Nana Osaki phiêu bạt tìm kiếm tự do trong nỗi nhớ khắc khoải.', 'Trưởng thành là hành trình đau đớn khi phải học cách chấp nhận sự khiếm khuyết của tình yêu và tìm thấy giá trị tự thân trước sóng gió cuộc đời.', datetime('now'), datetime('now')),
-- Fruits Basket (Natsuki Takaya · Nhật Bản · Lãng mạn / Shoujo) · chủ đề mẹ: Tình yêu, tận hiến và chữa lành · nguồn + [Shapes.inc: Fruits Basket Deep Dives - Thematic Analysis](https://shapes.inc/fandom/fruits-basket-2019/deep-dive)
-- PRIMARY: Tohru Honda (ACTION) · ANTITHESIS: Akito Sohma (ACTION & VERBAL) · CAUTIONARY: Cha mẹ của Kyo và các thành viên Sohma (OUTCOME) · WITNESS: Yuki Sohma & Kyo Sohma (IMPLICIT)
  ('018f0a03-0005-7000-8000-000000000005', 'THEME_FRUITS_BASKET_GENERATIONAL_HEALING', '018f0a00-0001-7000-8000-000000000008', 'Liệu những sang chấn thế hệ và xiềng xích gia tộc độc hại có thể bị hóa giải bằng lòng thấu cảm chân thành?', 'RESOLVED', 'Lời nguyền 12 con giáp tan vỡ khi các thành viên dám bước ra khỏi sự ràng buộc giả tạo để tự do yêu thương; Akito từ bỏ ngai vàng để học cách sống như con người.', 'Sự gắn kết thực sự không đến từ việc trói buộc hay áp đặt nghĩa vụ, mà nảy nở từ sự tự do yêu thương và tha thứ cho chính mình.', datetime('now'), datetime('now')),
-- Kimi ni Todoke (Karuho Shiina · Nhật Bản · Lãng mạn / Shoujo) · chủ đề mẹ: Tình yêu, tận hiến và chữa lành · nguồn + [TV Tropes: Kimi ni Todoke Manga Themes & Narrative](https://tvtropes.org/pmwiki/pmwiki.php/Manga/KimiNiTodoke)
-- PRIMARY: Kuronuma Sawako (ACTION) · ANTITHESIS: Ume Kurumizawa / Kurumi (ACTION & VERBAL) · CAUTIONARY: Những kẻ tung tin đồn vô căn cứ (OUTCOME) · WITNESS: Kazehaya Shouta & Chizuru / Ayane (ACTION)
  ('018f0a03-0006-7000-8000-000000000006', 'THEME_KIMI_NI_TODOKE_COMMUNICATION_AND_PREJUDICE', '018f0a00-0001-7000-8000-000000000008', 'Làm thế nào để tiếng nói của trái tim chạm tới người khác khi bị ngăn cách bởi bức tường định kiến diện mạo và sự tự ti?', 'RESOLVED', 'Sawako dùng sự chân thành và kiên trì giao tiếp để xóa tan hiểu lầm, tìm được những người bạn tri kỷ và tình yêu thanh xuân trọn vẹn với Kazehaya.', 'Chỉ khi dám gạt bỏ nỗi sợ bị chối bỏ và cất tiếng nói chân thật từ đáy lòng, con người mới có thể chạm tới tâm hồn tha nhân.', datetime('now'), datetime('now')),
-- Bến Xe (Thương Thái Vi · Trung Quốc · Lãng mạn / Bi kịch Ngôn tình) · chủ đề mẹ: Tình yêu, tận hiến và chữa lành · nguồn + [Fahasa Blog: Review Sách Bến Xe - Bản Tình Ca Bi Thương và Định Mệnh](https://www.fahasa.com/blog/review-sach-ben-xe-ban-tinh-ca-bi-thuong-ve-tinh-yeu-va-dinh-menh/)
-- PRIMARY: Chương Ngọc (ACTION & OUTCOME) · ANTITHESIS: Đám đông dư luận & Ban giám hiệu (VERBAL) · CAUTIONARY: Sự tàn nhẫn của định kiến thế tục (OUTCOME) · WITNESS: Liễu Địch (IMPLICIT)
  ('018f0a03-0007-7000-8000-000000000007', 'THEME_BEN_XE_SACRIFICE_VS_SOCIAL_PREJUDICE', '018f0a00-0001-7000-8000-000000000008', 'Cái giá của một tình cảm trong sáng trước sự suy đồi và định kiến nghiệt ngã của miệng lưỡi thế gian lớn đến mức nào?', 'RESOLVED', 'Chương Ngọc dùng tính mạng mình làm lời tuyên thệ đanh thép bảo vệ danh dự cho Liễu Địch, biến cái chết bi tráng thành ngọn hải đăng soi sáng tương lai cô.', 'Sự hy sinh cao thượng nhất trong tình yêu là chấp nhận lùi vào bóng tối để người mình yêu có thể bước tiếp dưới ánh mặt trời rạng rỡ.', datetime('now'), datetime('now')),
-- Cuốn Theo Chiều Gió (Margaret Mitchell · Hoa Kỳ · Lãng mạn / Sử thi Chiến tranh) · chủ đề mẹ: Tình yêu, tận hiến và chữa lành · nguồn + [LitCharts: Gone with the Wind Study Guide & Themes](https://www.litcharts.com/lit/gone-with-the-wind)
-- PRIMARY: Scarlett O'Hara (ACTION) · ANTITHESIS: Ashley Wilkes (VERBAL & OUTCOME) · CAUTIONARY: Rhett Butler (OUTCOME) · WITNESS: Melanie Wilkes (IMPLICIT)
  ('018f0a03-0008-7000-8000-000000000008', 'THEME_GONE_WITH_WIND_SURVIVAL_AND_ILLUSION', '018f0a00-0001-7000-8000-000000000008', 'Liệu con người có thể tìm thấy hạnh phúc khi mải miết theo đuổi một ảo ảnh quá khứ mà bỏ lỡ tình cảm chân thực bên cạnh?', 'OPEN', 'Scarlett nhận ra Ashley chỉ là ảo ảnh và Rhett mới là tình yêu đích thực, nhưng Rhett đã rời bỏ; cô quay về đất mẹ Tara với niềm tin ''Ngày mai là một ngày mới''.', 'Ảo tưởng có thể ru ngủ con người nhưng cũng bóp chết hạnh phúc hiện hữu; sức mạnh nội sinh là chiếc phao cứu sinh duy nhất khi bão tố thời đại cuốn phăng tất cả.', datetime('now'), datetime('now')),
-- Tát Dã (Vu Triết · Trung Quốc · Lãng mạn / Hiện thực Đam mỹ) · chủ đề mẹ: Tình yêu, tận hiến và chữa lành · nguồn + [WordPress Devonille: Cảm Nhận và Phân Tích Tiểu Thuyết Tát Dã](https://devonille.wordpress.com/2021/04/22/ve-tat-da/)
-- PRIMARY: Tưởng Thừa & Cố Phi (ACTION) · ANTITHESIS: Thị trấn xưởng thép mục ruỗng (OUTCOME) · CAUTIONARY: Lý Bảo Quốc (OUTCOME) · WITNESS: Cố Miểu (IMPLICIT)
  ('018f0a03-0009-7000-8000-000000000009', 'THEME_TAT_DA_REDEMPTION_AND_RESILIENCE', '018f0a00-0001-7000-8000-000000000008', 'Làm thế nào để hai linh hồn rạn nứt có thể vươn lên tự giải thoát khỏi nghịch cảnh tăm tối mà không đánh mất niềm tin vào cuộc sống?', 'RESOLVED', 'Tưởng Thừa thi đỗ đại học tại Bắc Kinh, kéo Cố Phi và Cố Miểu cùng bước ra khỏi vũng lầy thị trấn cũ, tìm lại hy vọng và tự do tung hoành (Tát Dã).', 'Dù bị quẳng vào nơi tối tăm nhất, con người vẫn có thể tìm thấy ánh sáng nếu có đủ ý chí sinh tồn và một tri kỷ nguyện cùng kề vai chiến đấu.', datetime('now'), datetime('now')),
-- Bloom Into You / Yagate Kimi ni Naru (Nio Nakatani · Nhật Bản · Lãng mạn / Bách hợp) · chủ đề mẹ: Tình yêu, tận hiến và chữa lành · nguồn + [TV Tropes: Bloom Into You (Yagate Kimi ni Naru) Themes & Characterization](https://tvtropes.org/pmwiki/pmwiki.php/Manga/BloomIntoYou)
-- PRIMARY: Koito Yuu (ACTION & VERBAL) · ANTITHESIS: Nanami Touko (ACTION) · CAUTIONARY: Saeki Sayaka (OUTCOME) · WITNESS: Maki Seiji (IMPLICIT)
  ('018f0a03-0010-7000-8000-000000000010', 'THEME_BLOOM_INTO_YOU_AUTHENTIC_IDENTITY', '018f0a00-0001-7000-8000-000000000008', 'Tình yêu là chấp nhận một hình tượng hoàn hảo đóng khung hay là dũng cảm đón nhận cả những phần yếu đuối, không hoàn hảo của nhau?', 'RESOLVED', 'Touko từ bỏ việc đóng vai người chị đã chết nhờ sự khích lệ chân thành của Yuu; cả hai cùng dũng cảm thổ lộ tình cảm thật và trưởng thành.', 'Chỉ khi dám gỡ bỏ mặt nạ và chấp nhận con người thật của mình, con người mới có thể cảm nhận trọn vẹn sự kỳ diệu và hơi ấm của tình yêu.', datetime('now'), datetime('now')),
-- Gintama (Hideaki Sorachi · Nhật Bản · Hài hước / Shounen Trào phúng) · chủ đề mẹ: Tiếng cười và sự phi lý · nguồn + [Goodreads: Quotes by Hideaki Sorachi - Gintama Philosophy](https://www.goodreads.com/author/quotes/700231.Hideaki_Sorachi)
-- PRIMARY: Sakata Gintoki (ACTION & VERBAL) · ANTITHESIS: Takasugi Shinsuke (ACTION) · CAUTIONARY: Madao / Hasegawa Taizou (OUTCOME) · WITNESS: Shinpachi Shimura & Kagura (IMPLICIT)
  ('018f0a03-0011-7000-8000-000000000011', 'THEME_GINTAMA_OPTIMISTIC_NIHILISM', '018f0a00-0001-7000-8000-000000000009', 'Khi thế giới trở nên vô lý và lý tưởng cao đẹp sụp đổ, con người nên chọn tuyệt vọng hủy diệt hay nở nụ cười kiên cường bước tiếp?', 'RESOLVED', 'Gintoki và các đồng đội bảo vệ thành công khu phố Kabuki, hóa giải hận thù với Takasugi, chứng minh tinh thần Samurai nằm ở ý chí bảo vệ những điều bình dị.', 'Nếu không thể có một cái chết lộng lẫy, hãy chọn sống một cách tử tế và tươi vui đến hơi thở cuối cùng bên cạnh những người mình yêu quý.', datetime('now'), datetime('now')),
-- KonoSuba (Natsume Akatsuki · Nhật Bản · Hài hước / Isekai Parody) · chủ đề mẹ: Tiếng cười và sự phi lý · nguồn + [AnimeAscension: KonoSuba Deconstructive Parody & Themes](https://www.animeascension.com/post/konosuba-god-s-blessing-on-this-wonderful-world)
-- PRIMARY: Satou Kazuma (ACTION) · ANTITHESIS: Mitsurugi Kyouya (VERBAL & OUTCOME) · CAUTIONARY: Bộ ba Aqua, Megumin, Darkness (ACTION) · WITNESS: Wiz & Vanir (IMPLICIT)
  ('018f0a03-0012-7000-8000-000000000012', 'THEME_KONOSUBA_DECONSTRUCTION_OF_HEROISM', '018f0a00-0001-7000-8000-000000000009', 'Liệu một cuộc đời ý nghĩa có nhất thiết phải là làm người hùng cứu thế vĩ đại, hay là việc tận hưởng niềm vui bên những kẻ lập dị bất toàn?', 'RESOLVED', 'Kazuma cùng nhóm bạn dở hơi đánh bại Quỷ Vương bằng những mưu mẹo thực dụng không giống ai, biến thế giới mới thành mái ấm ngập tràn tiếng cười.', 'Sự hoàn hảo chỉ là ảo ảnh nhàm chán; vẻ đẹp và niềm vui đích thực của cuộc sống nằm trong việc chấp nhận khiếm khuyết của nhau và cùng tạo nên tiếng cười giữa khó khăn.', datetime('now'), datetime('now')),
-- Don Quixote (Miguel de Cervantes · Tây Ban Nha · Hài hước / Trào phúng Cổ điển) · chủ đề mẹ: Tiếng cười và sự phi lý · nguồn + [LitCharts: Don Quixote Study Guide & Themes](https://www.litcharts.com/lit/don-quixote)
-- PRIMARY: Don Quixote (ACTION & VERBAL) · ANTITHESIS: Giới quý tộc và linh mục nông thôn (ACTION) · CAUTIONARY: Cuộc chiến cối xay gió & Sự thất bại trước Hiệp sĩ Ánh trăng Trắng (OUTCOME) · WITNESS: Sancho Panza (IMPLICIT)
  ('018f0a03-0013-7000-8000-000000000013', 'THEME_DON_QUIXOTE_IDEALISM_VS_REALITY', '018f0a00-0001-7000-8000-000000000009', 'Khi lý tưởng cao đẹp bị hiện thực tầm thường coi là điên rồ, kẻ mộng mơ nên tỉnh ngộ để hòa nhập hay tiếp tục giữ vững đức tin?', 'AMBIGUOUS', 'Don Quixote tỉnh lại trước khi qua đời, từ bỏ ảo mộng hiệp sĩ nhưng sự ra đi của ông lại để lại niềm tiếc thương sâu sắc về một tâm hồn cao thượng hiếm có.', 'Một chút điên rồ vì công lý và sự hào hiệp đáng trân trọng hơn ngàn lần sự tỉnh táo ích kỷ và thực dụng vô cảm của thế gian.', datetime('now'), datetime('now')),
-- Đại Quản Gia Là Ma Hoàng / Trọng Sinh Đô Thị Tu Tiên (Dạ Cú / Dạ Kiêu · Trung Quốc · Hài hước / Face-slapping Action) · chủ đề mẹ: Tiếng cười và sự phi lý · nguồn + [YouTube / Manhua Review: Đại Quản Gia Là Ma Hoàng & Nghệ Thuật Vả Mặt](https://www.youtube.com/watch?v=ZDn0YVzN9ac)
-- PRIMARY: Trác Nhất Phàm / Trác Phàm (ACTION) · ANTITHESIS: Thất đại thế gia & Hoàng quyền phong kiến (ACTION & VERBAL) · CAUTIONARY: Lãnh Vô Thường & Đế Vương Môn (OUTCOME) · WITNESS: Lạc Vân Thường & Sở Khuynh Thành (IMPLICIT)
  ('018f0a03-0014-7000-8000-000000000014', 'THEME_FACE_SLAPPING_SATIRE_PRAGMATISM', '018f0a00-0001-7000-8000-000000000009', 'Trước một trật tự xã hội thối nát nhân danh đạo đức để áp bức kẻ yếu, thái độ thực dụng phản anh hùng có phải là phương thức sinh tồn hiệu quả nhất?', 'RESOLVED', 'Trác Phàm dùng thủ đoạn ma đạo đưa Lạc Gia từ gia tộc hạng bét lên đỉnh cao quyền lực, đập tan mọi liên minh quý tộc đạo đức giả.', 'Chân lý không nằm ở những giáo điều đạo đức sáo rỗng được rao giảng bởi kẻ mạnh, mà ở hành động thực tế bảo vệ những người phụ thuộc vào mình.', datetime('now'), datetime('now')),
-- The Hitchhiker's Guide to the Galaxy (Douglas Adams · Anh Quốc · Hài hước / Khoa học Viễn tưởng Trào phúng) · chủ đề mẹ: Tiếng cười và sự phi lý · nguồn + [LitCharts: The Hitchhiker's Guide to the Galaxy Themes](https://www.litcharts.com/lit/the-hitchhiker-s-guide-to-the-galaxy/themes/meaninglessness-and-happiness)
-- PRIMARY: Arthur Dent (ACTION & VERBAL) · ANTITHESIS: Chủng tộc Vogon (ACTION) · CAUTIONARY: Marvin the Paranoid Android (OUTCOME) · WITNESS: Ford Prefect & Slartibartfast (IMPLICIT)
  ('018f0a03-0015-7000-8000-000000000015', 'THEME_HITCHHIKERS_ABSURDISM_AND_MEANING', '018f0a00-0001-7000-8000-000000000009', 'Làm thế nào để con người giữ được sự tỉnh táo và lạc quan khi nhận ra vũ trụ vốn dĩ hoàn toàn ngẫu nhiên và vô nghĩa?', 'OPEN', 'Câu trả lời cho Ý nghĩa Cuộc sống là con số 42 kỳ quặc; Arthur và các bạn tiếp tục cuộc hành trình ngân hà mà không cần đáp án tuyệt đối.', 'Trước sự mênh mông vô lý của vũ trụ, điều quan trọng nhất không phải là cố tìm kiếm một ý nghĩa vĩ mô cứng nhắc, mà là biết giữ bình tĩnh và trân trọng từng khoảnh khắc sống.', datetime('now'), datetime('now')),
-- Neon Genesis Evangelion (Hideaki Anno · Nhật Bản · Mecha / Tâm lý Học Tận thế) · chủ đề mẹ: Mecha và chiến tranh công nghệ · nguồn + [Wikipedia: Hedgehog's Dilemma in Neon Genesis Evangelion](https://en.wikipedia.org/wiki/Hedgehog%27s_Dilemma_%28Neon_Genesis_Evangelion)
-- PRIMARY: Ikari Shinji (ACTION & VERBAL) · ANTITHESIS: Ikari Gendo & SEELE (ACTION) · CAUTIONARY: Asuka Langley Soryu (OUTCOME) · WITNESS: Ayanami Rei & Misato Katsuragi (IMPLICIT)
  ('018f0a03-0016-7000-8000-000000000016', 'THEME_EVANGELION_HEDGEHOGS_DILEMMA', '018f0a00-0001-7000-8000-00000000000a', 'Liệu con người có nên xóa bỏ mọi ranh giới cá thể để thoát khỏi nỗi cô đơn, hay dũng cảm chấp nhận tổn thương để giữ lại bản ngã độc lập?', 'RESOLVED', 'Shinji hủy bỏ quá trình Instrumentality, chấp nhận quay trở lại thế giới thực tại nơi con người có thể làm đau nhau nhưng cũng có thể yêu thương nhau chân thật.', 'Sống là chấp nhận rủi ro bị tổn thương; sự tồn tại cá nhân chỉ có ý nghĩa khi ta dám bước qua nỗi sợ hãi để kết nối với người khác bằng chính bản ngã thật.', datetime('now'), datetime('now')),
-- Mobile Suit Gundam (Yoshiyuki Tomino · Nhật Bản · Mecha / Chiến tranh Hiện thực) · chủ đề mẹ: Mecha và chiến tranh công nghệ · nguồn + [Forbes: Yoshiyuki Tomino On 'Gundam', Newtypes And Perilous Future](https://www.forbes.com/sites/olliebarder/2017/03/13/yoshiyuki-tomino-on-gundam-newtypes-and-the-perilous-future-facing-humanity/)
-- PRIMARY: Amuro Ray (ACTION) · ANTITHESIS: Gihren Zabi & Phe Zeon cực đoan (VERBAL & ACTION) · CAUTIONARY: Char Aznable (OUTCOME) · WITNESS: Lalah Sune (IMPLICIT)
  ('018f0a03-0017-7000-8000-000000000017', 'THEME_GUNDAM_NEWTYPE_TRAGEDY_OF_WAR', '018f0a00-0001-7000-8000-00000000000a', 'Liệu sự tiến hóa tâm linh và công nghệ có thể mang lại hòa bình khi bản chất tranh đoạt quyền lực của con người không hề thay đổi?', 'OPEN', 'Cuộc chiến Một Năm kết thúc trong hoang tàn đổ nát; năng lực Newtype không cứu được Lalah nhưng mở ra tia hy vọng mong manh về sự thấu hiểu tương lai.', 'Vũ khí tối tân không thể thay thế cho sự thấu hiểu lẫn nhau; thảm kịch lớn nhất của chiến tranh là biến những tâm hồn khao khát kết nối thành cỗ máy giết người.', datetime('now'), datetime('now')),
-- Code Geass: Lelouch of the Rebellion (Gorō Taniguchi / Ichirō Ōkouchi · Nhật Bản · Mecha / Chính trị Bàn cờ) · chủ đề mẹ: Mecha và chiến tranh công nghệ · nguồn + [Scribd: Code Geass Character Analysis Research & Zero Requiem](https://www.scribd.com/document/1032951216/Code-Geass-Character-Analysis-Research)
-- PRIMARY: Lelouch vi Britannia (ACTION & OUTCOME) · ANTITHESIS: Charles zi Britannia & Schneizel (ACTION) · CAUTIONARY: Kururugi Suzaku (OUTCOME) · WITNESS: C.C. & Nunnally (IMPLICIT)
  ('018f0a03-0018-7000-8000-000000000018', 'THEME_CODE_GEASS_ZERO_REQUIEM_SACRIFICE', '018f0a00-0001-7000-8000-00000000000a', 'Một nền hòa bình bền vững có thể được xây dựng trên sự dối trá và máu của một kẻ độc tài tự nguyện làm vật tế thần hay không?', 'RESOLVED', 'Lelouch thực hiện Zero Requiem, để Suzaku trong lốt Zero đâm chết mình trước mắt toàn nhân loại, mở ra kỷ nguyên đàm phán và hợp tác quốc tế.', 'Kẻ nắm giữ quyền lực chỉ có quyền nổ súng khi chính bản thân sẵn sàng bị bắn hạ; sự cứu chuộc cao cả nhất của quyền lực là tự tiêu diệt để trao lại tương lai cho nhân dân.', datetime('now'), datetime('now')),
-- Tengen Toppa Gurren Lagann (Kazuki Nakashima / Hiroyuki Imaishi · Nhật Bản · Mecha / Siêu Cấp Tiến Hóa) · chủ đề mẹ: Mecha và chiến tranh công nghệ · nguồn + [Slant Magazine: Empowerment Fantasies - Tengen Toppa Gurren Lagann](https://www.slantmagazine.com/tv/empowerment-fantasies-tengen-toppa-gurren-lagann/)
-- PRIMARY: Simon & Kamina (ACTION & VERBAL) · ANTITHESIS: Anti-Spiral (ACTION) · CAUTIONARY: Rossiu Adai (OUTCOME) · WITNESS: Yoko Littner & Nia Teppelin (IMPLICIT)
  ('018f0a03-0019-7000-8000-000000000019', 'THEME_GURREN_LAGANN_HUMAN_WILL_AND_EVOLUTION', '018f0a00-0001-7000-8000-00000000000a', 'Nhân loại nên chấp nhận giam mình trong sự an toàn tù túng hay mạo hiểm vươn lên tiến hóa không ngừng dù có nguy cơ tự diệt?', 'RESOLVED', 'Simon đánh bại Anti-Spiral bằng niềm tin sắt đá, hứa hẹn sẽ kiểm soát năng lượng Spiral một cách có trách nhiệm rồi lui về làm một lữ khách vô danh.', 'Giới hạn sinh ra là để bị phá vỡ; ý chí của con người và niềm tin vào thế hệ mai sau là động lực vô tận giúp thắp sáng cả vũ trụ tăm tối.', datetime('now'), datetime('now')),
-- 86 - Eighty Six (Asato Asato · Nhật Bản · Mecha / Khoa học Viễn tưởng Quân sự) · chủ đề mẹ: Mecha và chiến tranh công nghệ · nguồn + [ResearchGate: Dehumanization and Identity Reclamation in 86 (Eighty-Six)](https://www.researchgate.net/publication/412357647_Dehumanization_and_Identity_Reclamation_in_86_Eighty-Six_A_Thematic_Analysis)
-- PRIMARY: Shinei Nouzen / Undertaker (ACTION) · ANTITHESIS: Cộng hòa San Magnolia / Giới chức quân sự Alba (ACTION & VERBAL) · CAUTIONARY: Legion (OUTCOME) · WITNESS: Vladilena Milizé / Lena (ACTION)
  ('018f0a03-0020-7000-8000-000000000020', 'THEME_EIGHTY_SIX_HUMAN_DIGNITY_VS_DEHUMANIZATION', '018f0a00-0001-7000-8000-00000000000a', 'Làm thế nào để con người giữ trọn nhân phẩm và lý do để sống khi bị xã hội tước đoạt quyền làm người và đẩy vào chỗ chết?', 'RESOLVED', 'Shin và biệt đội Spearhead sống sót vượt qua phòng tuyến diệt vong, hội ngộ với Lena tại Liên minh Giad trong tư cách những con người tự do có tên tuổi.', 'Không ai có quyền tước đoạt nhân phẩm của con người; chừng nào ta còn nhớ về người khác bằng tình yêu và danh dự, cái chết cũng không thể biến ta thành công cụ vô hồn.', datetime('now'), datetime('now')),
-- Assassination Classroom / Lớp Học Ám Sát (Yusei Matsui · Nhật Bản · Học viện / Giáo dục Siêu nhiên) · chủ đề mẹ: Giáo dục và trưởng thành · nguồn + [Academia.edu: Assassination Classroom - The Glimmer of Light in the Darkness](https://www.academia.edu/28264343/Assassination_Classroom_The_glimmer_of_light_in_the_darkness)
-- PRIMARY: Koro-sensei (ACTION & VERBAL) · ANTITHESIS: Hiệu trưởng Asano Gakuho (ACTION) · CAUTIONARY: Thần Chết thế hệ 2 / Reaper (OUTCOME) · WITNESS: Shiota Nagisa & Akabane Karma (ACTION)
  ('018f0a03-0021-7000-8000-000000000021', 'THEME_ASSASSINATION_CLASSROOM_EDUCATIONAL_PURPOSE', '018f0a00-0001-7000-8000-00000000000b', 'Mục đích cao quý nhất của giáo dục là tạo ra những cỗ máy thi cử thành tích cao hay là đánh thức giá trị tự thân của từng đứa trẻ bị tổn thương?', 'RESOLVED', 'Học sinh Lớp 3-E hoàn thành bài học cuối cùng bằng cách tự tay ám sát người thầy kính yêu trong nước mắt và bước vào đời với bản lĩnh vững vàng.', 'Người thầy vĩ đại không phải là người tạo ra những bản sao hoàn hảo, mà là người trao cho học trò niềm tin vào bản thân và dạy họ cách mỉm cười vượt qua thất bại.', datetime('now'), datetime('now')),
-- Dead Poets Society / Hội Cố Thi Nhân (N.H. Kleinbaum / Peter Weir · Hoa Kỳ · Học viện / Khai phóng Tư tưởng) · chủ đề mẹ: Giáo dục và trưởng thành · nguồn + [LitCharts: Dead Poets Society Themes - Carpe Diem & Conformity](https://www.litcharts.com/lit/dead-poets-society/themes)
-- PRIMARY: John Keating (ACTION & VERBAL) · ANTITHESIS: Ông Perry & Hiệu trưởng Nolan (ACTION) · CAUTIONARY: Neil Perry (OUTCOME) · WITNESS: Todd Anderson & các thành viên Hội Cố Thi Nhân (ACTION)
  ('018f0a03-0022-7000-8000-000000000022', 'THEME_DEAD_POETS_SOCIETY_CARPE_DIEM_VS_CONFORMITY', '018f0a00-0001-7000-8000-00000000000b', 'Liệu tuổi trẻ có nên thuận theo sự an toàn áp đặt của gia đình và xã hội hay dám dấn thân sống trọn vẹn với đam mê đích thực?', 'AMBIGUOUS', 'Keating bị sa thải sau cái chết của Neil, nhưng ngọn lửa tự do tư tưởng và tình yêu thi ca đã vĩnh viễn thắp sáng trong tâm hồn những học trò như Todd.', 'Y học, luật pháp, kinh tế là những điều cần thiết để duy trì cuộc sống, nhưng thi ca, cái đẹp và tình yêu mới chính là lý do khiến chúng ta sống trên đời.', datetime('now'), datetime('now')),
-- Blue Period (Tsubasa Yamaguchi · Nhật Bản · Học viện / Nghệ thuật Trưởng thành) · chủ đề mẹ: Giáo dục và trưởng thành · nguồn + [Medium: The Misconception between Hard Work and Talent in Blue Period](https://medium.com/@K.Ross_/the-misconception-between-hard-work-and-talent-blue-period-c4593f6dcce4)
-- PRIMARY: Yaguchi Yatora (ACTION) · ANTITHESIS: Quan niệm xã hội thực dụng (VERBAL) · CAUTIONARY: Ryuji / Yuka Ayukawa (OUTCOME) · WITNESS: Thầy Ooba & Cô Saeki (IMPLICIT)
  ('018f0a03-0023-7000-8000-000000000023', 'THEME_BLUE_PERIOD_ART_IDENTITY_AND_EFFORT', '018f0a00-0001-7000-8000-00000000000b', 'Nghệ thuật là đặc quyền của thiên tài bẩm sinh hay là kết quả của sự dũng cảm phơi bày vết thương tâm hồn và nỗ lực lao động phi thường?', 'RESOLVED', 'Yatora vượt qua cơn đau thể xác và khủng hoảng tâm lý để thi đỗ Đại học Nghệ thuật Tokyo (Geidai) bằng bức tranh thể hiện góc nhìn chân thật của chính mình.', 'Năng khiếu chỉ là điểm khởi đầu; chính sự kiên trì đối thoại với nỗi sợ và lòng can đảm bộc lộ chân thật con người mình mới tạo nên giá trị nghệ thuật đích thực.', datetime('now'), datetime('now')),
-- Boku no Hero Academia (Kohei Horikoshi · Nhật Bản · Học viện / Siêu Anh Hùng Shounen) · chủ đề mẹ: Giáo dục và trưởng thành · nguồn + [ScreenRant: My Hero Academia's Creator Revealed the True Theme](https://screenrant.com/my-hero-academia-creator-revealed-true-theme-connect-entire-story/)
-- PRIMARY: Midoriya Izuku / Deku (ACTION) · ANTITHESIS: Shigaraki Tomura & All For One (ACTION) · CAUTIONARY: Endeavor / Todoroki Enji (OUTCOME) · WITNESS: All Might & Tập thể Lớp 1-A (ACTION & VERBAL)
  ('018f0a03-0024-7000-8000-000000000024', 'THEME_BNHA_SHARED_HEROISM_AND_RESPONSIBILITY', '018f0a00-0001-7000-8000-00000000000b', 'Một xã hội công lý nên dựa vào một siêu anh hùng cá nhân gánh vác tất cả hay là sự chung tay kết nối trách nhiệm của toàn bộ cộng đồng?', 'RESOLVED', 'Deku cùng toàn thể bạn bè và dân thường cùng nhau đứng lên đánh bại All For One, xác lập kỷ nguyên ''mọi người đều có thể trở thành anh hùng của ai đó''.', 'Chủ nghĩa anh hùng vĩ đại nhất không nằm ở sức mạnh vô song của một cá nhân, mà ở việc dang tay giúp đỡ người khác trong hoạn nạn và cùng nhau gánh vác tương lai.', datetime('now'), datetime('now')),
-- Kakegurui (Homura Kawamoto · Nhật Bản · Học viện / Tâm lý Cờ bạc Đen tối) · chủ đề mẹ: Giáo dục và trưởng thành · nguồn + [AIPT Comics: Kakegurui - Compulsive Gambler Thematic Review](https://aiptcomics.com/2018/03/09/kakegurui-compulsive-gambler-vol-1-review/)
-- PRIMARY: Yumeko Jabami (ACTION) · ANTITHESIS: Kirari Momobami & Hội Học Sinh (ACTION) · CAUTIONARY: Mary Saotome & Kaede Manyuda (OUTCOME) · WITNESS: Ryota Suzui (IMPLICIT)
  ('018f0a03-0025-7000-8000-000000000025', 'THEME_KAKEGURUI_CAPITALISM_AND_RISK_MADNESS', '018f0a00-0001-7000-8000-00000000000b', 'Khi một hệ thống học đường biến tiền bạc và ranh giới sinh tử thành thước đo giá trị con người, kẻ tỉnh táo hay kẻ điên cuồng mới là người tự do?', 'OPEN', 'Yumeko liên tục xáo trộn trật tự của Hội Học Sinh mà không hề có tham vọng chính trị, biến cả học viện thành sàn diễn bộc lộ bản chất trần trụi của lòng tham.', 'Khi một xã hội tôn thờ giá trị đồng tiền đến mức phi nhân tính, mọi ảo tưởng về trật tự đạo đức sẽ bị xé toạc trước bản năng cuồng loạn và sự bất định của số phận.', datetime('now'), datetime('now')),
-- Barakamon (Satsuki Yoshino · Nhật Bản · Đời thường & Điền văn / Hài hước Chữa lành) · chủ đề mẹ: Sống chậm và sinh thái · nguồn + [GameRant: Best Anime About Art - Barakamon Analysis](https://gamerant.com/best-anime-about-art/)
-- PRIMARY: Handa Seishuu (ACTION) · ANTITHESIS: Giám đốc Triển lãm Thư pháp (VERBAL) · CAUTIONARY: Áp lực kỳ vọng gia đình & Thành công sớm (OUTCOME) · WITNESS: Naru Kotoishi & Dân đảo Goto (ACTION)
  ('018f0a03-0026-7000-8000-000000000026', 'THEME_BARAKAMON_RURAL_HEALING_AND_CREATIVITY', '018f0a00-0001-7000-8000-00000000000c', 'Liệu sự hoàn hảo kỹ thuật hay cảm xúc sống động từ trải nghiệm đời thường mới là linh hồn thực sự của nghệ thuật sáng tạo?', 'RESOLVED', 'Handa trở lại Tokyo chấp nhận thất bại ở triển lãm mà không oán trách, nhận ra phong cách của riêng mình và quyết định quay lại đảo gắn bó lâu dài.', 'Để tạo nên những tác phẩm rung động lòng người, nghệ sĩ trước hết phải học cách sống một cuộc đời chân thật, biết yêu thương và hòa mình vào cuộc sống bình dị.', datetime('now'), datetime('now')),
-- Silver Spoon / Gin no Saji (Hiromu Arakawa · Nhật Bản · Đời thường & Điền văn / Nông nghiệp Thực tế) · chủ đề mẹ: Sống chậm và sinh thái · nguồn + [Anime Monographia: Requiem for a Pork Bowl - Sentimental Pragmatism in Silver Spoon](https://animemonographia.wordpress.com/2013/12/22/requiem-for-a-pork-bowl-sentimental-pragmatism-in-silver-spoon/)
-- PRIMARY: Hachiken Yuugo (ACTION) · ANTITHESIS: Người cha hà khắc của Hachiken (VERBAL) · CAUTIONARY: Gia đình Komaba bị phá sản (OUTCOME) · WITNESS: Mikage Aki & Các giáo viên trường Ooezo (IMPLICIT)
  ('018f0a03-0027-7000-8000-000000000027', 'THEME_SILVER_SPOON_AGRICULTURE_LIFE_AND_DEATH', '018f0a00-0001-7000-8000-00000000000c', 'Làm thế nào để một thanh thiếu niên mất phương hướng tìm lại ý nghĩa cuộc sống khi trực tiếp đối diện với mồ hôi lao động và cái chết của sinh vật nuôi?', 'RESOLVED', 'Hachiken thành lập công ty khởi nghiệp nông nghiệp để giúp đỡ bạn bè, tìm thấy mục tiêu sống và dũng cảm đối thoại bình đẳng với cha mình.', 'Không có thức ăn nào tự nhiên mà có; hiểu được cái giá của sinh mạng bị tước đoạt trên bàn ăn là bước đầu tiên để một con người học cách trân trọng sự sống của chính mình.', datetime('now'), datetime('now')),
-- Mushishi (Yuki Urushibara · Nhật Bản · Đời thường & Điền văn / Huyền ảo Sinh thái) · chủ đề mẹ: Sống chậm và sinh thái · nguồn + [Academia.edu: Mushishi's Apparitional Actants - Humanity & Ecological Coexistence](https://www.academia.edu/83789659/Past_the_End_of_the_Catbus_Line_Mushishi_s_Apparitional_Actants)
-- PRIMARY: Ginko (ACTION) · ANTITHESIS: Những người dân làng sợ hãi hoặc tham lam (ACTION) · CAUTIONARY: Những thảm kịch sinh thái (OUTCOME) · WITNESS: Adashino & Tanyuu Karibusa (IMPLICIT)
  ('018f0a03-0028-7000-8000-000000000028', 'THEME_MUSHISHI_COEXISTENCE_WITH_NATURE', '018f0a00-0001-7000-8000-00000000000c', 'Khi sự tồn tại của tự nhiên đe dọa đến cuộc sống con người, ta nên chọn tiêu diệt để sinh tồn hay tìm cách thấu hiểu để cùng chung sống?', 'OPEN', 'Ginko tiếp tục hành trình vô tận qua các cánh rừng, không bao giờ ở lại một chỗ quá lâu, nhắc nhở con người về vị trí khiêm nhường của mình trong chuỗi sinh thái.', 'Mushi không phải là ác quỷ, chúng chỉ đơn thuần đang sống; sự tôn trọng đa dạng sinh học và thấu cảm với vạn vật là cội nguồn của sự bình yên trong tâm hồn.', datetime('now'), datetime('now')),
-- Nông Gia Tiểu Phúc Nữ (Úc Vũ Trúc · Trung Quốc · Đời thường & Điền văn / Làm ruộng Cổ đại) · chủ đề mẹ: Sống chậm và sinh thái · nguồn + [Wattpad: Nông Gia Tiểu Phúc Nữ - Úc Vũ Trúc & Triết Lý Điền Văn](https://www.wattpad.com/story/287127798-edit-ph%C3%BAc-n%E1%BB%AF-nh%C3%A0-n%C3%B4ng-%C3%BAc-v%C5%A9-tr%C3%BAc)
-- PRIMARY: Chu Mãn Bảo (ACTION) · ANTITHESIS: Lệ làng cổ hủ & Tư tưởng trọng nam khinh nữ (VERBAL) · CAUTIONARY: Quan lại tham nhũng & Phú hào bức ép nông dân (OUTCOME) · WITNESS: Bạch Thiện & Tiên sinh Trang (ACTION)
  ('018f0a03-0029-7000-8000-000000000029', 'THEME_NONG_GIA_TIEL_PHUC_NU_INDUSTRY_AND_KNOWLEDGE', '018f0a00-0001-7000-8000-00000000000c', 'Làm thế nào để một gia đình bần nông thoát nghèo bền vững giữa thời phong kiến hà khắc mà vẫn giữ gìn được đạo đức và tình làng nghĩa xóm?', 'RESOLVED', 'Chu Mãn trở thành nữ thái y và nhà nông học lỗi lạc, cải tạo đất đai và đưa cả gia tộc họ Chu bước vào tầng lớp danh giá có học thức.', 'Sự cần cù siêng năng kết hợp với tri thức khoa học và lòng hiếu thảo chính là phúc khí vững bền nhất giúp con người làm chủ vận mệnh của mình.', datetime('now'), datetime('now')),
-- Yuru Camp / Laid-Back Camp (Afro · Nhật Bản · Đời thường & Điền văn / Iyashikei Cắm trại) · chủ đề mẹ: Sống chậm và sinh thái · nguồn + [Instagram: Best Cozy Anime - Laid-Back Camp (Yuru Camp) & Slow Living](https://www.instagram.com/popular/best-cozy-anime/)
-- PRIMARY: Shima Rin (ACTION) · ANTITHESIS: Áp lực cuộc sống bận rộn hối hả nơi phố thị (IMPLICIT) · CAUTIONARY: Thiếu chuẩn bị an toàn khi ra thiên nhiên (OUTCOME) · WITNESS: Kagamihara Nadeshiko & Câu lạc bộ Hoạt động ngoài trời (ACTION)
  ('018f0a03-0030-7000-8000-000000000030', 'THEME_YURU_CAMP_SLOW_LIVING_AND_SOLITUDE', '018f0a00-0001-7000-8000-00000000000c', 'Làm thế nào để con người hiện đại tìm lại sự cân bằng tâm lý giữa guồng quay hối hả của xã hội số?', 'RESOLVED', 'Rin và Nadeshiko vừa duy trì sở thích cắm trại cá nhân vừa tận hưởng những chuyến dã ngoại chung cùng nhóm bạn, tìm thấy sự an yên dưới chân núi Phú Sĩ.', 'Hạnh phúc không nằm ở những điều lớn lao xa vời, mà hiện diện trong khoảnh khắc tĩnh lặng ngắm bình minh, húp một bát mì nóng và sẻ chia hơi ấm giữa thiên nhiên bao la.', datetime('now'), datetime('now'))
ON CONFLICT (code) DO UPDATE SET
  parent_theme_id = EXCLUDED.parent_theme_id, question = EXCLUDED.question,
  resolution_type = EXCLUDED.resolution_type, resolution = EXCLUDED.resolution,
  message = EXCLUDED.message, updated_at = datetime('now');

-- 0021: Mọi tool được cấp cho prompt phase phải có description.
--
-- `PromptComposer::get_tools_by_prompt` (chat_orchestrator.rs) loại bỏ bất kỳ tool nào có
-- description rỗng. Trước migration này chỉ 12/87 tool có description, nên dù
-- `prompt_block_tools` cấp 75 tool cho `planning_phase_0`, model chỉ nhận được 12 tool:
-- không có add_arc/add_theme/add_theme_argument/add_relationship_role/add_scene_config/get_state...
-- Hệ quả: model không đọc được dàn ý Arc, không biết schema của role/quan hệ và tự bịa summary.
--
-- Migration bổ sung description cho toàn bộ tool còn thiếu, đồng thời khai báo enum cho các
-- trường dễ sai (role, answer_mode, hierarchy, các chiều quan hệ, is_pov).

UPDATE tools SET description = CASE code
  -- Arc
  WHEN 'add_arc' THEN 'Thêm một Arc vào optional.overview_phase.arcs[]. Tham số: code?, display_name, summary, position, declared_sources.'
  WHEN 'set_arc' THEN 'Cập nhật Arc đã có theo code. Tham số: code, display_name?, summary?, position?, declared_sources.'
  WHEN 'delete_arc' THEN 'Xóa Arc theo code; cascade chương và cảnh thuộc Arc sau khi kiểm tra source.'
  WHEN 'set_arc_motif' THEN 'Gán motif xung đột cho Arc. Tham số: code, motif_code?, note?, declared_sources.'
  WHEN 'add_arc_event_block' THEN 'Thêm khối biến cố cho Arc. Tham số: arc_code, code?, event_block_code?, start_chapter?, end_chapter?, summary?, declared_sources.'
  WHEN 'delete_arc_event_block' THEN 'Xóa một khối biến cố của Arc theo code.'
  -- Theme
  WHEN 'add_theme' THEN 'Thêm chủ đề cho Arc. Tham số: arc_code, code?, parent_theme_code?, question?, resolution_type?, resolution?, message?, declared_sources.'
  WHEN 'set_theme' THEN 'Cập nhật chủ đề của Arc theo code. Tham số: code, question?, resolution?, message?, declared_sources.'
  WHEN 'delete_theme' THEN 'Xóa chủ đề của Arc theo code.'
  WHEN 'add_theme_argument' THEN 'Thêm lập trường nhân vật cho chủ đề. Tham số: theme_code, character_code?, role (PRIMARY|ANTITHESIS|CAUTIONARY|WITNESS), answer_mode (VERBAL|ACTION|OUTCOME|IMPLICIT), note?, declared_sources.'
  WHEN 'delete_theme_argument' THEN 'Xóa một lập trường của chủ đề theo code.'
  -- Chương, cảnh
  WHEN 'add_chapter' THEN 'Thêm chương vào Arc. Tham số: arc_code, code?, display_name, position, summary, declared_sources. position là thứ tự toàn truyện.'
  WHEN 'set_chapter' THEN 'Cập nhật chương theo code. Tham số: code, display_name?, position?, summary?, declared_sources.'
  WHEN 'delete_chapter' THEN 'Xóa chương theo code; cascade cảnh thuộc chương.'
  WHEN 'add_scene' THEN 'Thêm cảnh thô vào chương. Tham số: chapter_code, code?, display_name, position, summary, declared_sources.'
  WHEN 'set_scene' THEN 'Cập nhật cảnh theo code. Tham số: code, display_name?, position?, summary?, declared_sources.'
  WHEN 'delete_scene' THEN 'Xóa cảnh theo code; cascade các liên kết nhân vật và lore của cảnh.'
  WHEN 'set_scene_config' THEN 'Cấu hình kể chuyện cho cảnh. Tham số: scene_code, style_preset_code?, narrator_config_code?, world_setting_code?, declared_sources.'
  WHEN 'add_scene_character' THEN 'Gắn nhân vật vào cảnh. Tham số: scene_code, character_code, is_pov?, declared_sources.'
  WHEN 'delete_scene_character' THEN 'Gỡ nhân vật khỏi cảnh. Tham số: scene_code, character_code, declared_source.'
  WHEN 'add_scene_lore_entry' THEN 'Gắn lore vào cảnh. Tham số: scene_code, lore_entry_code, usage?, reveal_to_reader?, reveal_to_pov?, declared_sources.'
  WHEN 'set_scene_lore_entry' THEN 'Cập nhật liên kết lore của cảnh (usage, mức tiết lộ).'
  WHEN 'delete_scene_lore_entry' THEN 'Gỡ liên kết lore khỏi cảnh. Tham số: scene_code, lore_entry_code, declared_source.'
  -- Bối cảnh, lore
  WHEN 'add_world_setting' THEN 'Thêm nút cây bối cảnh. Tham số: code?, parent_code?, display_name, type (REALM|WORLD|REGION|COUNTRY|CITY|TOWN|FACILITY|STREET|ROOM), description?, access?, declared_sources.'
  WHEN 'set_world_setting' THEN 'Cập nhật nút bối cảnh theo code. Tham số: code, display_name?, type?, description?, access?, parent_code?, declared_sources.'
  WHEN 'delete_world_setting' THEN 'Xóa nút bối cảnh; bị chặn nếu còn cảnh tham chiếu (referenced_entity_in_use).'
  WHEN 'add_lore_entry' THEN 'Thêm mục tri thức gắn bối cảnh. Tham số: code?, world_setting_code, display_name, type (CULTURE|HISTORY|FACTION|ITEM), description?, data?, declared_sources.'
  WHEN 'set_lore_entry' THEN 'Cập nhật mục tri thức theo code. Tham số: code, display_name?, type?, description?, data?, declared_sources.'
  WHEN 'delete_lore_entry' THEN 'Xóa mục tri thức; cascade liên kết scene_lore_entries sau khi kiểm tra source.'
  -- Nhân vật
  WHEN 'add_character' THEN 'Thêm nhân vật vào require.character[] và gắn dự án. Tham số: display_name?, nickname?, biological_sex?, motif_code?, project_role?, project_notes?, declared_sources.'
  WHEN 'set_character' THEN 'Cập nhật nhân vật theo code. Tham số: code, display_name?, nickname?, species?, birth?, gender_identity?, gender_expression?, declared_sources.'
  WHEN 'delete_character' THEN 'Xóa nhân vật; bị chặn nếu còn quan hệ tham chiếu (referenced_entity_in_use).'
  WHEN 'set_project_character' THEN 'Cập nhật liên kết nhân vật với dự án (vai trò, ghi chú).'
  WHEN 'add_character_motif' THEN 'Gán motif cho nhân vật. Tham số: character_code, motif_code, declared_source.'
  WHEN 'delete_character_motif' THEN 'Gỡ motif khỏi nhân vật. Tham số: character_code, motif_code, declared_source.'
  WHEN 'add_character_appearance' THEN 'Thêm hồ sơ ngoại hình (5.1). Tham số: character_code, species?, height?, weight?, hair_color_code?, skin_tone_code?, outfit_style?, default_summary?, declared_sources.'
  WHEN 'set_character_appearance' THEN 'Cập nhật hồ sơ ngoại hình theo character_code.'
  WHEN 'add_character_distinguishing_mark' THEN 'Thêm dấu hiệu nhận dạng. Tham số: character_code, location?, content, declared_source.'
  WHEN 'delete_character_distinguishing_mark' THEN 'Xóa dấu hiệu nhận dạng của nhân vật.'
  WHEN 'add_character_psychological' THEN 'Thêm hồ sơ tâm lý (5.2): the_wound, the_lie, fundamental_fear, moral_alignment, external_want, internal_need, internal_conflict, social_mask, defense_mechanisms; declared_sources.'
  WHEN 'set_character_psychological' THEN 'Cập nhật hồ sơ tâm lý theo character_code.'
  WHEN 'add_character_memory' THEN 'Thêm ký ức nhân vật (5.3). Tham số: character_code, code?, display_name?, summary?, source_scene_code?, declared_sources.'
  WHEN 'set_character_memory' THEN 'Cập nhật ký ức theo code.'
  WHEN 'delete_character_memory' THEN 'Xóa ký ức của nhân vật.'
  WHEN 'add_character_competence' THEN 'Thêm năng lực nhân vật (5.3). Tham số: character_code, code?, domain_code?, level_code?, scene_code (bắt buộc), capabilities?, limitations?, declared_sources.'
  WHEN 'set_character_competence' THEN 'Cập nhật năng lực theo code.'
  WHEN 'delete_character_competence' THEN 'Xóa năng lực của nhân vật.'
  WHEN 'add_competence_capability' THEN 'Thêm khả năng cho một năng lực.'
  WHEN 'delete_competence_capability' THEN 'Xóa khả năng của năng lực.'
  WHEN 'add_competence_limitation' THEN 'Thêm giới hạn cho một năng lực.'
  WHEN 'delete_competence_limitation' THEN 'Xóa giới hạn của năng lực.'
  -- Quan hệ
  WHEN 'add_relationship' THEN 'Tạo quan hệ có hướng. Tham số: code?, from_code, to_code, motif_code?, declared_sources.'
  WHEN 'set_relationship' THEN 'Cập nhật quan hệ theo code.'
  WHEN 'delete_relationship' THEN 'Xóa quan hệ theo code.'
  WHEN 'add_relationship_role' THEN 'Thêm vai trò xã hội cho quan hệ. Tham số: relationship_code, category_code?, hierarchy (SUPERIOR|EQUAL|INFERIOR), is_primary?, declared_sources.'
  WHEN 'delete_relationship_role' THEN 'Xóa vai trò của quan hệ.'
  WHEN 'set_relationship_state' THEN 'Ghi trạng thái quan hệ tại cảnh. Tham số: relationship_code, scene_code, trust?, affection?, hostility?, fear?, respect?, familiarity?, dependency? (NONE|LOW|MODERATE|HIGH|ABSOLUTE), declared_sources.'
  -- Đọc, tra cứu
  WHEN 'get_state' THEN 'Đọc State bản nháp theo thực thể hoặc toàn bộ; trả kèm source và patch đang chờ. Tham số: target_type?, code?, fields?.'
  WHEN 'get_tool_definition' THEN 'Đọc mô tả và input_schema của một tool đã được cấp. Tham số: tool_name.'
  WHEN 'get_field_null' THEN 'Trả tối đa 3 trường còn null kèm câu hỏi cố định. Tham số: limit?.'
  WHEN 'get_array_element_fields' THEN 'Trả trường còn thiếu của phần tử mảng hoặc trường cần tạo phần tử mới. Tham số: array_path, code?, limit?.'
  WHEN 'lookup_outline' THEN 'Tra cứu mục lục chương/cảnh theo trang. Tham số: cursor?, limit?.'
  WHEN 'search_character' THEN 'Tìm nhân vật theo tên hoặc biệt danh. Tham số: query, limit?.'
  WHEN 'search_genre' THEN 'Tra cứu thể loại theo mã hoặc tên hiển thị trong danh mục; dùng mã trả về khi gắn thể loại.'
  WHEN 'add_project_genre' THEN 'Gắn thể loại đã xác minh vào dự án; genre_code phải là mã có thật, declared_sources.genre_code khai báo nguồn.'
  WHEN 'delete_project_genre' THEN 'Gỡ thể loại khỏi dự án. Tham số: genre_code, declared_source.'
  WHEN 'search_motif' THEN 'Tra cứu mô-típ theo mã, tên hoặc nhóm; dùng mã đã tra cứu khi gán mô-típ.'
  WHEN 'search_event_block' THEN 'Tra cứu khối biến cố theo mã hoặc tên.'
  WHEN 'search_style_preset' THEN 'Tra cứu phong cách diễn đạt theo mã hoặc tên.'
  WHEN 'search_narrator_config' THEN 'Tra cứu cấu hình người kể chuyện theo mã hoặc tên.'
  WHEN 'search_appearance_option' THEN 'Tra cứu lựa chọn ngoại hình (màu tóc, màu da, mắt) theo nhóm.'
  WHEN 'search_relationship_category' THEN 'Tra cứu nhóm quan hệ theo mã hoặc tên.'
  WHEN 'search_competence_domain' THEN 'Tra cứu lĩnh vực năng lực theo mã hoặc tên.'
  WHEN 'add_project' THEN 'Thêm dự án mới vào bản nháp. Tham số: display_name?, summary?, language?, target?, declared_sources.'
  WHEN 'set_project' THEN 'Cập nhật thông tin dự án. Tham số: display_name?, summary?, language?, target?, declared_sources.'
  ELSE NULL
END
WHERE trim(coalesce(description, '')) = ''
  AND code IN (
    'add_arc','set_arc','delete_arc','set_arc_motif','add_arc_event_block','delete_arc_event_block',
    'add_theme','set_theme','delete_theme','add_theme_argument','delete_theme_argument',
    'add_chapter','set_chapter','delete_chapter','add_scene','set_scene','delete_scene',
    'set_scene_config','add_scene_character','delete_scene_character','add_scene_lore_entry',
    'set_scene_lore_entry','delete_scene_lore_entry','add_world_setting','set_world_setting',
    'delete_world_setting','add_lore_entry','set_lore_entry','delete_lore_entry',
    'add_character','set_character','delete_character','set_project_character','add_character_motif',
    'delete_character_motif','add_character_appearance','set_character_appearance',
    'add_character_distinguishing_mark','delete_character_distinguishing_mark',
    'add_character_psychological','set_character_psychological','add_character_memory',
    'set_character_memory','delete_character_memory','add_character_competence',
    'set_character_competence','delete_character_competence','add_competence_capability',
    'delete_competence_capability','add_competence_limitation','delete_competence_limitation',
    'add_relationship','set_relationship','delete_relationship','add_relationship_role',
    'delete_relationship_role','set_relationship_state','get_state','get_tool_definition',
    'get_field_null','get_array_element_fields','lookup_outline','search_character','search_genre',
    'add_project_genre','delete_project_genre','search_motif','search_event_block',
    'search_style_preset','search_narrator_config','search_appearance_option',
    'search_relationship_category','search_competence_domain','add_project','set_project'
  );

-- Mọi tool còn lại (nếu có) nhận mô tả tối thiểu để không bị bộ lọc loại bỏ.
UPDATE tools
SET description = 'Tool ' || code || ': tham số theo tài liệu phase và docs/db.dbml; '
               || 'khai báo declared_sources (hoặc declared_source với tool xóa) cho mọi trường ghi.'
WHERE trim(coalesce(description, '')) = '';

-- Enum tường minh cho các trường hay sai (schema vẫn permissive để backend tự validate).
UPDATE tools SET input_schema = json_set(
  input_schema,
  '$.properties.role', json('{"type":"string","enum":["PRIMARY","ANTITHESIS","CAUTIONARY","WITNESS"],"description":"Vai trò lập trường của nhân vật trong chủ đề"}'),
  '$.properties.answer_mode', json('{"type":"string","enum":["VERBAL","ACTION","OUTCOME","IMPLICIT"],"description":"Cách nhân vật trả lời chủ đề"}'),
  '$.properties.theme_code', json('{"type":"string","minLength":1}'),
  '$.properties.character_code', json('{"type":"string","minLength":1}')
)
WHERE code = 'add_theme_argument' AND json_valid(input_schema);

UPDATE tools SET input_schema = json_set(
  input_schema,
  '$.properties.hierarchy', json('{"type":"string","enum":["SUPERIOR","EQUAL","INFERIOR"],"description":"Thứ bậc quyền lực của vai trò"}'),
  '$.properties.is_primary', json('{"type":"boolean"}'),
  '$.properties.relationship_code', json('{"type":"string","minLength":1}')
)
WHERE code = 'add_relationship_role' AND json_valid(input_schema);

UPDATE tools SET input_schema = json_set(
  input_schema,
  '$.properties.trust', json('{"type":"string","enum":["NONE","LOW","MODERATE","HIGH","ABSOLUTE"]}'),
  '$.properties.affection', json('{"type":"string","enum":["NONE","LOW","MODERATE","HIGH","ABSOLUTE"]}'),
  '$.properties.hostility', json('{"type":"string","enum":["NONE","LOW","MODERATE","HIGH","ABSOLUTE"]}'),
  '$.properties.fear', json('{"type":"string","enum":["NONE","LOW","MODERATE","HIGH","ABSOLUTE"]}'),
  '$.properties.respect', json('{"type":"string","enum":["NONE","LOW","MODERATE","HIGH","ABSOLUTE"]}'),
  '$.properties.familiarity', json('{"type":"string","enum":["NONE","LOW","MODERATE","HIGH","ABSOLUTE"]}'),
  '$.properties.dependency', json('{"type":"string","enum":["NONE","LOW","MODERATE","HIGH","ABSOLUTE"]}'),
  '$.properties.relationship_code', json('{"type":"string","minLength":1}'),
  '$.properties.scene_code', json('{"type":"string","minLength":1}')
)
WHERE code = 'set_relationship_state' AND json_valid(input_schema);

UPDATE tools SET input_schema = json_set(
  input_schema,
  '$.properties.is_pov', json('{"type":"boolean","description":"Nhân vật giữ góc nhìn; ngôi kể giới hạn cần đúng một is_pov"}'),
  '$.properties.scene_code', json('{"type":"string","minLength":1}'),
  '$.properties.character_code', json('{"type":"string","minLength":1}')
)
WHERE code = 'add_scene_character' AND json_valid(input_schema);

-- 0022: Durable handoff of an author-approved outline (`save_approved_outline`).
--
-- Chat may present a proposed outline as prose. When the author approves it, the model writes the
-- approved text into `BusinessFlowData.decision_log[]` through this tool. The workflow composes
-- its prompts from the draft snapshot (it has no chat history), so `decision_log` is the handoff
-- that lets Phase 1 honour the approved outline instead of regenerating one from scratch.

INSERT OR IGNORE INTO tools (id, code, display_name, description, input_schema) VALUES (
  '5f0c2f0a-3a57-4b2f-9f43-6a1e3c8d2b71',
  'save_approved_outline',
  'Lưu dàn ý đã được tác giả duyệt',
  'Ghi dàn ý hoặc quyết định đã được tác giả duyệt trong chat vào decision_log để các phase sau (workflow) đọc lại. Gọi khi tác giả xác nhận một dàn ý/quyết định; summary là toàn bộ nội dung đã duyệt, declared_sources.summary = user.',
  '{"type":"object","additionalProperties":false,"properties":{"summary":{"type":"string","minLength":1,"description":"Toàn bộ nội dung dàn ý/quyết định tác giả đã duyệt"},"declared_sources":{"type":"object","properties":{"summary":{"type":"string","enum":["user","ai_generated"]}},"required":["summary"],"additionalProperties":false}},"required":["summary","declared_sources"]}'
);

-- Every planning phase can persist an approval; the tool writes a planning decision, not chapter
-- content, so it is never granted to Writing Mode.
INSERT OR IGNORE INTO prompt_block_tools (id, prompt_block_id, tool_id)
SELECT
  lower(hex(randomblob(4))) || '-' || lower(hex(randomblob(2))) || '-' ||
  lower(hex(randomblob(2))) || '-' || lower(hex(randomblob(2))) || '-' || lower(hex(randomblob(6))),
  pb.id, t.id
FROM prompt_blocks pb
JOIN tools t ON t.code = 'save_approved_outline'
WHERE pb.prompt_code IN (
  'planning_phase_0', 'planning_phase_1', 'planning_phase_2', 'planning_phase_3',
  'planning_phase_4', 'planning_phase_5', 'planning_phase_6'
);

-- Phase 0 instruction: capture the approved outline instead of dropping it after the turn.
UPDATE prompt_blocks
SET content = content || char(10) || 'Khi tác giả duyệt một dàn ý, danh sách quyết định hoặc một thoả thuận cần giữ lại, gọi tool save_approved_outline với summary là toàn bộ nội dung đã duyệt và declared_sources.summary = user. Không chỉ trả lời bằng lời: phase sau (workflow) không đọc được lịch sử chat.'
WHERE prompt_code = 'planning_phase_0';

-- Phase 1 instruction: read the durable approval before writing arcs.
UPDATE prompt_blocks
SET content = content || char(10) || 'Trước khi viết Arc, đọc decision_log trong story context. Nếu có dàn ý đã được tác giả duyệt (save_approved_outline), arc/chương phải nhất quán với dàn ý đó thay vì tạo lại từ đầu.'
WHERE prompt_code = 'planning_phase_1';

-- 0023: Real function-calling schemas for planning tools.
--
-- Most planning tools were seeded with `{"type":"object","properties":{}}`. The model could not
-- see parameter names (add_arc had no motif_code, set_arc_motif had no parameters at all), so
-- Phase 1 generated Arcs without a motif and the phase gate kept the draft stuck. This migration
-- gives every implemented mutation tool a real schema and adds pagination to catalog list tools.

UPDATE tools SET input_schema = '{"type":"object","properties":{"code":{"type":"string"},"display_name":{"type":"string"},"summary":{"type":"string"},"position":{"type":"integer","minimum":1},"motif_code":{"type":"string"},"motif_note":{"type":"string"},"declared_sources":{"type":"object","additionalProperties":{"type":"string","enum":["user","ai_generated"]}}},"required":["display_name","declared_sources"]}' WHERE code='add_arc';

UPDATE tools SET input_schema = '{"type":"object","properties":{"code":{"type":"string"},"display_name":{"type":"string"},"summary":{"type":"string"},"position":{"type":"integer","minimum":1},"declared_sources":{"type":"object","additionalProperties":{"type":"string","enum":["user","ai_generated"]}}},"required":["code","declared_sources"]}' WHERE code='set_arc';

UPDATE tools SET input_schema = '{"type":"object","properties":{"code":{"type":"string"},"motif_code":{"type":"string"},"note":{"type":"string"},"declared_sources":{"type":"object","properties":{"motif_code":{"type":"string","enum":["user","ai_generated"]},"note":{"type":"string","enum":["user","ai_generated"]}}}},"required":["code","motif_code","declared_sources"]}' WHERE code='set_arc_motif';

UPDATE tools SET input_schema = '{"type":"object","properties":{"arc_code":{"type":"string"},"code":{"type":"string"},"event_block_code":{"type":"string"},"start_chapter":{"type":"integer","minimum":1},"end_chapter":{"type":"integer","minimum":1},"summary":{"type":"string"},"declared_sources":{"type":"object","additionalProperties":{"type":"string","enum":["user","ai_generated"]}}},"required":["arc_code","declared_sources"]}' WHERE code='add_arc_event_block';

UPDATE tools SET input_schema = '{"type":"object","properties":{"arc_code":{"type":"string"},"theme_code":{"type":"string"},"parent_theme_code":{"type":"string"},"question":{"type":"string"},"resolution_type":{"type":"string"},"resolution":{"type":"string"},"message":{"type":"string"},"declared_sources":{"type":"object","additionalProperties":{"type":"string","enum":["user","ai_generated"]}}},"required":["arc_code","declared_sources"]}' WHERE code='add_theme';

UPDATE tools SET input_schema = '{"type":"object","properties":{"theme_code":{"type":"string"},"character_code":{"type":"string"},"role":{"type":"string","enum":["PRIMARY","ANTITHESIS","CAUTIONARY","WITNESS"]},"answer_mode":{"type":"string"},"note":{"type":"string"},"declared_sources":{"type":"object","additionalProperties":{"type":"string","enum":["user","ai_generated"]}}},"required":["theme_code","declared_sources"]}' WHERE code='add_theme_argument';

UPDATE tools SET input_schema = '{"type":"object","properties":{"arc_code":{"type":"string"},"code":{"type":"string"},"display_name":{"type":"string"},"position":{"type":"integer","minimum":1},"summary":{"type":"string"},"declared_sources":{"type":"object","additionalProperties":{"type":"string","enum":["user","ai_generated"]}}},"required":["arc_code","display_name","declared_sources"]}' WHERE code='add_chapter';

UPDATE tools SET input_schema = '{"type":"object","properties":{"code":{"type":"string"},"display_name":{"type":"string"},"position":{"type":"integer","minimum":1},"summary":{"type":"string"},"declared_sources":{"type":"object","additionalProperties":{"type":"string","enum":["user","ai_generated"]}}},"required":["code","declared_sources"]}' WHERE code='set_chapter';

UPDATE tools SET input_schema = '{"type":"object","properties":{"chapter_code":{"type":"string"},"code":{"type":"string"},"display_name":{"type":"string"},"position":{"type":"integer","minimum":1},"summary":{"type":"string"},"declared_sources":{"type":"object","additionalProperties":{"type":"string","enum":["user","ai_generated"]}}},"required":["chapter_code","display_name","declared_sources"]}' WHERE code='add_scene';

UPDATE tools SET input_schema = '{"type":"object","properties":{"code":{"type":"string"},"display_name":{"type":"string"},"position":{"type":"integer","minimum":1},"summary":{"type":"string"},"world_setting_code":{"type":"string"},"declared_sources":{"type":"object","additionalProperties":{"type":"string","enum":["user","ai_generated"]}}},"required":["code","declared_sources"]}' WHERE code='set_scene';

UPDATE tools SET input_schema = '{"type":"object","properties":{"scene_code":{"type":"string"},"style_preset_code":{"type":"string"},"narrator_config_code":{"type":"string"},"world_setting_code":{"type":"string"},"declared_sources":{"type":"object","additionalProperties":{"type":"string","enum":["user","ai_generated"]}}},"required":["scene_code","declared_sources"]}' WHERE code='set_scene_config';

UPDATE tools SET input_schema = '{"type":"object","properties":{"scene_code":{"type":"string"},"character_code":{"type":"string"},"is_pov":{"type":"boolean"},"declared_sources":{"type":"object","additionalProperties":{"type":"string","enum":["user","ai_generated"]}}},"required":["scene_code","character_code","declared_sources"]}' WHERE code='add_scene_character';

UPDATE tools SET input_schema = '{"type":"object","properties":{"scene_code":{"type":"string"},"lore_entry_code":{"type":"string"},"usage":{"type":"string"},"reveal_to_reader":{"type":"boolean"},"reveal_to_pov":{"type":"boolean"},"declared_sources":{"type":"object","additionalProperties":{"type":"string","enum":["user","ai_generated"]}}},"required":["scene_code","lore_entry_code","declared_sources"]}' WHERE code='add_scene_lore_entry';

UPDATE tools SET input_schema = '{"type":"object","properties":{"code":{"type":"string"},"parent_code":{"type":"string"},"display_name":{"type":"string"},"type":{"type":"string","enum":["REALM","WORLD","REGION","COUNTRY","CITY","TOWN","FACILITY","STREET","ROOM"]},"description":{"type":"string"},"access":{"type":"string"},"declared_sources":{"type":"object","additionalProperties":{"type":"string","enum":["user","ai_generated"]}}},"required":["display_name","type","declared_sources"]}' WHERE code='add_world_setting';

UPDATE tools SET input_schema = '{"type":"object","properties":{"code":{"type":"string"},"display_name":{"type":"string"},"type":{"type":"string","enum":["REALM","WORLD","REGION","COUNTRY","CITY","TOWN","FACILITY","STREET","ROOM"]},"description":{"type":"string"},"access":{"type":"string"},"declared_sources":{"type":"object","additionalProperties":{"type":"string","enum":["user","ai_generated"]}}},"required":["code","declared_sources"]}' WHERE code='set_world_setting';

UPDATE tools SET input_schema = '{"type":"object","properties":{"code":{"type":"string"},"world_setting_code":{"type":"string"},"display_name":{"type":"string"},"type":{"type":"string","enum":["CULTURE","HISTORY","FACTION","ITEM"]},"description":{"type":"string"},"data":{"type":"object"},"declared_sources":{"type":"object","additionalProperties":{"type":"string","enum":["user","ai_generated"]}}},"required":["world_setting_code","display_name","type","declared_sources"]}' WHERE code='add_lore_entry';

UPDATE tools SET input_schema = '{"type":"object","properties":{"code":{"type":"string"},"display_name":{"type":"string"},"type":{"type":"string","enum":["CULTURE","HISTORY","FACTION","ITEM"]},"description":{"type":"string"},"data":{"type":"object"},"declared_sources":{"type":"object","additionalProperties":{"type":"string","enum":["user","ai_generated"]}}},"required":["code","declared_sources"]}' WHERE code='set_lore_entry';

UPDATE tools SET input_schema = '{"type":"object","properties":{"code":{"type":"string"},"from_code":{"type":"string"},"to_code":{"type":"string"},"motif_code":{"type":"string"},"declared_sources":{"type":"object","additionalProperties":{"type":"string","enum":["user","ai_generated"]}}},"required":["from_code","to_code","declared_sources"]}' WHERE code='add_relationship';

UPDATE tools SET input_schema = '{"type":"object","properties":{"relationship_code":{"type":"string"},"scene_code":{"type":"string"},"trust":{"type":"string","enum":["NONE","LOW","MODERATE","HIGH","ABSOLUTE"]},"affection":{"type":"string","enum":["NONE","LOW","MODERATE","HIGH","ABSOLUTE"]},"hostility":{"type":"string","enum":["NONE","LOW","MODERATE","HIGH","ABSOLUTE"]},"fear":{"type":"string","enum":["NONE","LOW","MODERATE","HIGH","ABSOLUTE"]},"respect":{"type":"string","enum":["NONE","LOW","MODERATE","HIGH","ABSOLUTE"]},"dependency":{"type":"string","enum":["NONE","LOW","MODERATE","HIGH","ABSOLUTE"]},"familiarity":{"type":"string","enum":["NONE","LOW","MODERATE","HIGH","ABSOLUTE"]},"declared_sources":{"type":"object","additionalProperties":{"type":"string","enum":["user","ai_generated"]}}},"required":["relationship_code","scene_code","declared_sources"]}' WHERE code='set_relationship_state';

UPDATE tools SET input_schema = '{"type":"object","properties":{"character_code":{"type":"string"},"code":{"type":"string"},"display_name":{"type":"string"},"summary":{"type":"string"},"source_scene_code":{"type":"string"},"declared_sources":{"type":"object","additionalProperties":{"type":"string","enum":["user","ai_generated"]}}},"required":["character_code","declared_sources"]}' WHERE code='add_character_memory';

UPDATE tools SET input_schema = '{"type":"object","properties":{"character_code":{"type":"string"},"code":{"type":"string"},"domain_code":{"type":"string"},"level_code":{"type":"string"},"scene_code":{"type":"string"},"display_name":{"type":"string"},"description":{"type":"string"},"capabilities":{"type":"array","items":{"type":"string"}},"limitations":{"type":"array","items":{"type":"string"}},"declared_sources":{"type":"object","additionalProperties":{"type":"string","enum":["user","ai_generated"]}}},"required":["character_code","scene_code","declared_sources"]}' WHERE code='add_character_competence';

UPDATE tools SET input_schema = '{"type":"object","properties":{"summary":{"type":"string"},"declared_sources":{"type":"object","properties":{"summary":{"type":"string","enum":["user","ai_generated"]}},"required":["summary"]}},"required":["summary","declared_sources"]}' WHERE code='save_approved_outline';

-- `set_project` keeps its richer contract from 0005 (examples + target tolerances); only the
-- unseeded `add_project` alias is filled here.
UPDATE tools SET input_schema = '{"type":"object","properties":{"display_name":{"type":"string"},"summary":{"type":"string"},"language":{"type":"string","enum":["VI","EN","JA","ZH_HANT","ZH_HANS"]},"target":{"type":"object","properties":{"arc_count":{"type":"integer"},"chapter_count":{"type":"integer"},"word_per_chapter_count":{"type":"integer"}}},"declared_sources":{"type":"object","additionalProperties":{"type":"string","enum":["user","ai_generated"]}}},"required":["declared_sources"]}' WHERE code='add_project';

UPDATE tools SET input_schema = '{"type":"object","properties":{"genre_code":{"type":"string"},"is_primary":{"type":"boolean"},"declared_sources":{"type":"object","properties":{"genre_code":{"type":"string","enum":["user","ai_generated"]},"is_primary":{"type":"string","enum":["user","ai_generated"]}},"required":["genre_code"]}},"required":["genre_code","declared_sources"]}' WHERE code='add_project_genre';

-- Catalog list tools: code-only pages plus detail-by-code.
UPDATE tools SET input_schema = '{"type":"object","properties":{"code":{"type":"string","description":"Return full details for this exact code."},"query":{"type":"string","description":"Optional case-insensitive name/code filter."},"limit":{"type":"integer","minimum":1,"maximum":100,"default":20},"offset":{"type":"integer","minimum":0,"default":0}},"additionalProperties":false}' WHERE code='search_genre';

UPDATE tools SET input_schema = '{"type":"object","properties":{"code":{"type":"string","description":"Return full details for this exact code."},"query":{"type":"string","description":"Optional case-insensitive name/code filter."},"category":{"type":"string","description":"Optional motif category, e.g. character_arc."},"limit":{"type":"integer","minimum":1,"maximum":100,"default":20},"offset":{"type":"integer","minimum":0,"default":0}},"additionalProperties":false}' WHERE code='search_motif';

UPDATE tools SET input_schema = '{"type":"object","properties":{"code":{"type":"string"},"query":{"type":"string"},"limit":{"type":"integer","minimum":1,"maximum":100,"default":20},"offset":{"type":"integer","minimum":0,"default":0}},"additionalProperties":false}' WHERE code IN ('search_character','search_event_block','search_narrator_config','search_style_preset','search_competence_domain','search_relationship_category','search_appearance_option');

UPDATE tools SET input_schema = '{"type":"object","properties":{"character_code":{"type":"string"},"species":{"type":"string"},"birth":{"type":"string"},"height_cm":{"type":"integer","minimum":1},"weight_kg":{"type":"integer","minimum":1},"outfit_style":{"type":"string"},"default_summary":{"type":"string"},"declared_sources":{"type":"object","additionalProperties":{"type":"string","enum":["user","ai_generated"]}}},"required":["character_code","declared_sources"]}' WHERE code IN ('add_character_appearance','set_character_appearance');

UPDATE tools SET input_schema = '{"type":"object","properties":{"character_code":{"type":"string"},"the_wound":{"type":"string"},"the_lie":{"type":"string"},"fundamental_fear":{"type":"string"},"external_want":{"type":"string"},"internal_need":{"type":"string"},"internal_conflict":{"type":"string"},"declared_sources":{"type":"object","additionalProperties":{"type":"string","enum":["user","ai_generated"]}}},"required":["character_code","declared_sources"]}' WHERE code IN ('add_character_psychological','set_character_psychological');

-- 0024: fix the add_project_genre schema published by 0023.
--
-- 0023 declared the scalar `declared_source`, but the executor's SourceParamMode is
-- MapDeclaredSources and reads `declared_sources.genre_code`. The model followed the schema,
-- every genre write was rejected with `missing_or_invalid_declared_source`, project.genres stayed
-- empty and the readiness gate kept the draft closed even though get_field_null returned no gaps.

UPDATE tools SET input_schema = '{"type":"object","properties":{"genre_code":{"type":"string"},"is_primary":{"type":"boolean"},"declared_sources":{"type":"object","properties":{"genre_code":{"type":"string","enum":["user","ai_generated"]},"is_primary":{"type":"string","enum":["user","ai_generated"]}},"required":["genre_code"]}},"required":["genre_code","declared_sources"]}' WHERE code='add_project_genre';

-- 0025: the loaded business state JSON is the SSOT; verify every write against it.
--
-- The model called add_project_genre, received a rejection, never asked again, and continued as
-- if genres were saved because it trusted its own tool call rather than the state JSON. This rule
-- applies to chat and workflow prompts (the introduction block is shared by both).

UPDATE prompt_blocks
SET content = content || char(10) || char(10) ||
  'Toàn bộ JSON trạng thái nghiệp vụ (business_flow_data/WorkingState) được nạp trong context là nguồn sự thật duy nhất (SSOT). Không được giả định một lần gọi tool là đã ghi thành công: sau mỗi lần ghi, đọc kết quả tool (status/error) và đối chiếu lại state mới nhất. Nếu một trường bắt buộc vẫn null hoặc một mảng bắt buộc vẫn rỗng trong SSOT (ví dụ get_field_null trả empty_arrays/array_questions), phải hỏi lại tác giả hoặc sửa và gọi lại tool trong cùng lượt; không được tuyên bố đã lưu xong rồi chuyển bước.'
WHERE prompt_code = 'introduction';

UPDATE prompt_blocks
SET content = content || char(10) || char(10) ||
  'Nếu kết quả tool là rejected, đọc kỹ trường lỗi, sửa tham số theo schema rồi thử lại; không bỏ qua lỗi và không coi dữ liệu đã được lưu. Chỉ khi SSOT đã có dữ liệu hợp lệ mới được nói với tác giả là đã lưu.'
WHERE prompt_code = 'planning_phase_0';

-- 0026: expose the full 5.2 psychological contract in function-calling schemas.
-- The handler accepted only six fields, so breaking_point/moral_code sent by the model were dropped.

UPDATE tools SET input_schema = '{"type":"object","properties":{"character_code":{"type":"string"},"the_wound":{"type":"string"},"the_lie":{"type":"string"},"fundamental_fear":{"type":"string"},"external_want":{"type":"string"},"internal_need":{"type":"string"},"internal_conflict":{"type":"string"},"moral_alignment":{"type":"string"},"moral_code":{"type":"string"},"cognitive_bias":{"type":"string"},"breaking_point":{"type":"string"},"social_mask_persona_type":{"type":"string"},"social_mask_verbal_style":{"type":"string"},"defense_primary_reaction":{"type":"string"},"defense_stress_trigger":{"type":"string"},"defense_coping_strategy":{"type":"string"},"declared_sources":{"type":"object","additionalProperties":{"type":"string","enum":["user","ai_generated"]}}},"required":["character_code","declared_sources"]}' WHERE code IN ('add_character_psychological','set_character_psychological');

PRAGMA user_version = 2;
