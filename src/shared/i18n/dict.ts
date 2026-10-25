const en = {
  'common.cancel': 'Cancel',
  'common.save': 'Save',
  'common.saved': 'Saved',
  'common.delete': 'Delete',
  'common.deleted': 'Deleted',
  'common.edit': 'Edit',
  'common.confirm': 'Confirm',
  'common.close': 'Close',

  'discard.title': 'Discard changes?',
  'discard.description': 'You have unsaved changes. Are you sure you want to discard them?',
  'discard.confirm': 'Discard',
  'discard.cancel': 'Keep editing',

  'delete.title': 'Confirm deletion',
  'delete.description': 'This action cannot be undone. Type the 6-character code below to confirm.',
  'delete.codeLabel': 'Type the code below to confirm:',
  'delete.codePlaceholder': 'Type 6-character code',
  'delete.codeMismatch': 'Code does not match',
  'delete.confirm': 'Delete',

  'toast.featureInProgress': 'This feature is being completed',

  'settings.title': 'Settings',
  'settings.subtitle': 'Customize your environment',
  'settings.general': 'General',
  'settings.aiProviders': 'AI Providers',
  'settings.general.title': 'General Settings',
  'settings.general.subtitle': 'Manage appearance and language preferences.',
  'settings.language': 'Language',
  'settings.language.en': 'English',
  'settings.language.vi': 'Vietnamese',
  'settings.appearance': 'Appearance',
  'settings.appearance.light': 'Light',
  'settings.appearance.dark': 'Dark',
  'settings.appearance.system': 'System',

  'providers.models': 'Models',
  'providers.subtitle':
    'Enter your API keys to use models from the following providers.',
  'providers.add': 'Add model provider',
  'providers.custom': 'Custom',
  'providers.signedIn': 'Signed in',
  'providers.edit': 'Edit',
  'providers.delete': 'Delete',
  'providers.delete.title': 'Delete provider',
  'providers.delete.description': 'This provider will be permanently removed.',

  'project.title': 'Project',
  'project.subtitle': 'Manage your world scripts and text progress.',
  'project.searchPlaceholder': 'Search projects or genres...',
  'project.sort.a-z': 'Title (A - Z)',
  'project.sort.z-a': 'Title (Z - A)',
  'project.sort.modified': 'Modified Date',
  'project.empty': 'No matching projects found.',
  'project.overview': 'Overview',
  'project.delete.title': 'Move project to recycle bin',
  'project.delete.description':
    'This project will be moved to the recycle bin. You can restore it later.',
  'project.trashTooltip': 'Move to recycle bin',
  'project.deletedToast': 'Project moved to recycle bin',

  'chat.mode.chat': 'Chat',
  'chat.mode.manual': 'Manual',
  'chat.draftBuilding': 'Draft building',
  'chat.preview': 'preview',
} as const;

const vi: Record<keyof typeof en, string> = {
  'common.cancel': 'Hủy',
  'common.save': 'Lưu',
  'common.saved': 'Đã lưu',
  'common.delete': 'Xóa',
  'common.deleted': 'Đã xóa',
  'common.edit': 'Chỉnh sửa',
  'common.confirm': 'Xác nhận',
  'common.close': 'Đóng',

  'discard.title': 'Hủy thay đổi?',
  'discard.description': 'Bạn có thay đổi chưa lưu. Bạn có chắc muốn hủy không?',
  'discard.confirm': 'Hủy thay đổi',
  'discard.cancel': 'Tiếp tục chỉnh sửa',

  'delete.title': 'Xác nhận xóa',
  'delete.description':
    'Hành động này không thể hoàn tác. Nhập mã 6 ký tự bên dưới để xác nhận.',
  'delete.codeLabel': 'Nhập mã bên dưới để xác nhận:',
  'delete.codePlaceholder': 'Nhập mã 6 ký tự',
  'delete.codeMismatch': 'Mã không khớp',
  'delete.confirm': 'Xóa',

  'toast.featureInProgress': 'Chức năng đang được hoàn thiện',

  'settings.title': 'Cài đặt',
  'settings.subtitle': 'Tùy biến môi trường',
  'settings.general': 'Chung',
  'settings.aiProviders': 'Nhà cung cấp AI',
  'settings.general.title': 'Cài đặt chung',
  'settings.general.subtitle': 'Quản lý giao diện và ngôn ngữ.',
  'settings.language': 'Ngôn ngữ',
  'settings.language.en': 'Tiếng Anh',
  'settings.language.vi': 'Tiếng Việt',
  'settings.appearance': 'Giao diện',
  'settings.appearance.light': 'Sáng',
  'settings.appearance.dark': 'Tối',
  'settings.appearance.system': 'Hệ thống',

  'providers.models': 'Mô hình',
  'providers.subtitle':
    'Nhập API key để dùng mô hình từ các nhà cung cấp bên dưới.',
  'providers.add': 'Thêm nhà cung cấp',
  'providers.custom': 'Tùy chỉnh',
  'providers.signedIn': 'Đã đăng nhập',
  'providers.edit': 'Sửa',
  'providers.delete': 'Xóa',
  'providers.delete.title': 'Xóa nhà cung cấp',
  'providers.delete.description': 'Nhà cung cấp này sẽ bị xóa vĩnh viễn.',

  'project.title': 'Dự án',
  'project.subtitle': 'Quản lý các kịch bản thế giới và tiến trình văn bản của bạn.',
  'project.searchPlaceholder': 'Tìm dự án hoặc thể loại...',
  'project.sort.a-z': 'Tên (A - Z)',
  'project.sort.z-a': 'Tên (Z - A)',
  'project.sort.modified': 'Ngày sửa đổi',
  'project.empty': 'Không tìm thấy dự án phù hợp.',
  'project.overview': 'Xem nhanh',
  'project.delete.title': 'Chuyển dự án vào thùng rác',
  'project.delete.description':
    'Dự án sẽ được chuyển vào thùng rác. Bạn có thể phục hồi sau.',
  'project.trashTooltip': 'Chuyển vào thùng rác',
  'project.deletedToast': 'Đã chuyển dự án vào thùng rác',

  'chat.mode.chat': 'Trò chuyện',
  'chat.mode.manual': 'Thủ công',
  'chat.draftBuilding': 'Xây dựng bản nháp',
  'chat.preview': 'xem trước',
};

export const dict = { en, vi };
export type Locale = keyof typeof dict;
export type TKey = keyof typeof en;
