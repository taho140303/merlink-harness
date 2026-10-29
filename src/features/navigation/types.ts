export interface ChatItem {
  id: string;
  title: string;
  time: string;
  projectId: string; // 3.1: Lưu project_id theo DBML (dùng cho query/deep-link)
  projectName: string; // Chỉ dùng để hiển thị trên giao diện
}
