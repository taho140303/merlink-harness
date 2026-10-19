/* Merlink Harness · local-only sample content and cross-page React components */
(function (global) {
  'use strict';
  const React = global.React;
  const h = React.createElement;
  const HOME = 'index.html';
  const storage = {
    get(key, fallback) { try { return global.localStorage.getItem(key) ?? fallback; } catch (_) { return fallback; } },
    set(key, value) { try { global.localStorage.setItem(key, value); } catch (_) {} }
  };
  function initialTheme() { const value = storage.get('merlink-theme', 'system'); return ['light', 'dark', 'system'].includes(value) ? value : 'system'; }
  function resolveTheme(value) { return value === 'system' ? (global.matchMedia?.('(prefers-color-scheme: dark)').matches ? 'dark' : 'light') : value; }
  function applyTheme(value) {
    const preference = ['light', 'dark', 'system'].includes(value) ? value : 'system';
    const theme = resolveTheme(preference);
    document.documentElement.dataset.theme = theme;
    document.documentElement.classList.toggle('dark', theme === 'dark');
    storage.set('merlink-theme', preference);
    global.dispatchEvent(new CustomEvent('merlink:theme', { detail: preference }));
  }
  applyTheme(initialTheme());
  if (global.matchMedia) global.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', () => { if (initialTheme() === 'system') applyTheme('system'); });
  function useTheme() {
    const [theme, setCurrent] = React.useState(initialTheme);
    React.useEffect(() => {
      const sync = () => setCurrent(initialTheme());
      const custom = event => setCurrent(event.detail);
      global.addEventListener('storage', sync);
      global.addEventListener('merlink:theme', custom);
      return () => { global.removeEventListener('storage', sync); global.removeEventListener('merlink:theme', custom); };
    }, []);
    function setTheme(next) { const value = typeof next === 'function' ? next(initialTheme()) : next; applyTheme(value); setCurrent(value); }
    return { theme, setTheme, toggleTheme: () => setTheme({ system: 'dark', dark: 'light', light: 'system' }[initialTheme()]) };
  }
  function useSidebar() {
    const [collapsed, setCollapsed] = React.useState(() => storage.get('merlink-sidebar-collapsed', 'false') === 'true');
    React.useEffect(() => {
      const onStorage = event => { if (event.key === 'merlink-sidebar-collapsed') setCollapsed(event.newValue === 'true'); };
      global.addEventListener('storage', onStorage);
      return () => global.removeEventListener('storage', onStorage);
    }, []);
    function setSidebar(next) { setCollapsed(previous => { const value = typeof next === 'function' ? next(previous) : next; storage.set('merlink-sidebar-collapsed', String(value)); return value; }); }
    return { collapsed, setCollapsed: setSidebar, toggleSidebar: () => setSidebar(value => !value) };
  }
  const ICON_ATTRIBUTES = { class: 'className', 'stroke-width': 'strokeWidth', 'stroke-linecap': 'strokeLinecap', 'stroke-linejoin': 'strokeLinejoin', 'fill-rule': 'fillRule', 'clip-rule': 'clipRule', 'xlink:href': 'xlinkHref' };
  function iconElement(node, key) {
    if (!Array.isArray(node)) return null;
    const [tag, attrs, children] = node;
    const props = { key };
    Object.entries(attrs || {}).forEach(([name, value]) => { props[ICON_ATTRIBUTES[name] || name] = value; });
    return h(tag, props, ...(children || []).map((child, index) => iconElement(child, index)));
  }
  function Icon({ name, size = 20, className = '', ...rest }) {
    const icons = global.lucide && global.lucide.icons;
    const pascal = String(name || '').replace(/(^|[-_\s])([a-z])/g, (_, prefix, letter) => letter.toUpperCase());
    const node = icons && (icons[name] || icons[pascal]);
    if (!node) return h('span', { className, role: 'img', 'aria-label': String(name || 'icon'), style: { display: 'inline-flex', width: size, height: size, flexShrink: 0 } });
    return h('svg', { xmlns: 'http://www.w3.org/2000/svg', width: size, height: size, viewBox: '0 0 24 24', fill: 'none', stroke: 'currentColor', strokeWidth: 2, strokeLinecap: 'round', strokeLinejoin: 'round', className, 'aria-hidden': 'true', ...rest }, ...node.map((part, index) => iconElement(part, index)));
  }
  function ThemeToggle() {
    const { theme, toggleTheme } = useTheme();
    return h('button', { type: 'button', className: 'btn merlink-theme-toggle', title: `Theme: ${theme}. Change theme`, 'aria-label': `Theme: ${theme}. Change theme`, onClick: toggleTheme }, h(Icon, { name: theme === 'system' ? 'Monitor' : theme === 'dark' ? 'Moon' : 'Sun', size: 18 }));
  }
  const navigation = [
    { href: 'chat.html', label: 'Chat', icon: 'MessageSquare', key: 'chat' },
    { href: 'projects.html', label: 'Projects', icon: 'BookOpen', key: 'projects' },
    { href: 'characters.html', label: 'Characters', icon: 'Users', key: 'characters' },
    { href: 'story-config.html', label: 'Story Config', icon: 'Settings2', key: 'story-config' }
  ];
  function Atmosphere() {
    return h('div', { className: 'merlink-atmosphere', 'aria-hidden': 'true' },
      ...[1,2,3,4].map(index => h('div', { key: index, className: 'merlink-blob' })),
      h('svg', { className: 'merlink-grain', xmlns: 'http://www.w3.org/2000/svg', width: '100%', height: '100%', preserveAspectRatio: 'none' },
        h('defs', null, h('filter', { id: 'merlink-noise' }, h('feTurbulence', { type: 'fractalNoise', baseFrequency: '.78', numOctaves: '3', stitchTiles: 'stitch' }))),
        h('rect', { width: '100%', height: '100%', filter: 'url(#merlink-noise)', opacity: '.64' })
      )
    );
  }
  function Shell({ active, children, wide = false, noMaxWidth = false, fullWidth = false, sidebarExtra = null, sidebarDrawer = false, onCloseSidebarDrawer }) {
    const { collapsed, setCollapsed, toggleSidebar } = useSidebar();
    const drawerRef = React.useRef(null);
    const previousFocus = React.useRef(null);
    const [settingsOpen, setSettingsOpen] = React.useState(false);
    const [settingsTab, setSettingsTab] = React.useState('general');
    React.useEffect(() => {
      const requested = storage.get('merlink-open-settings-on-load', '');
      if (requested === 'llm-provider' || requested === 'general') {
        global.localStorage.removeItem('merlink-open-settings-on-load');
        setSettingsTab(requested);
        setSettingsOpen(true);
      }
    }, []);
    React.useEffect(() => {
      if (!sidebarDrawer) return;
      previousFocus.current = document.activeElement;
      drawerRef.current?.focus();
      const onKey = event => { if (event.key === 'Escape') onCloseSidebarDrawer?.(); };
      document.addEventListener('keydown', onKey);
      return () => { document.removeEventListener('keydown', onKey); if (previousFocus.current?.isConnected) previousFocus.current.focus(); };
    }, [sidebarDrawer]);
    return h('div', { className: `merlink-app${collapsed ? ' is-collapsed' : ''}${sidebarExtra ? ' has-sidebar-extra' : ''}${sidebarDrawer ? ' sidebar-drawer-open' : ''}` },
      sidebarDrawer && h('button', { type:'button', className:'merlink-sidebar-backdrop', 'aria-label':'Close conversations', onClick:onCloseSidebarDrawer }),
      h(Atmosphere),
      h('aside', { ref: drawerRef, tabIndex: sidebarDrawer ? -1 : undefined, className: 'merlink-sidebar surface', 'aria-label': 'Primary navigation' },
        h('div', { className: 'merlink-brand' },
          h('a', { className: 'merlink-brand-home', href: HOME, title: 'Merlink Harness home', 'aria-label': 'Merlink Harness home' },
            h('span', { className: 'merlink-brand-mark' }, h('img', { src: 'logo.png', width: '36', height: '36', alt: '' })),
            h('span', { className: 'merlink-brand-text' }, h('span', { className: 'merlink-brand-title' }, 'Merlink Harness'), h('span', { className: 'merlink-brand-version' }, 'v1.0 STORY TOOL'))
          ),
          h('button', { type: 'button', className: 'merlink-collapse', onClick: toggleSidebar, title: 'Collapse sidebar', 'aria-label': 'Collapse sidebar' }, h(Icon, { name: 'PanelLeftClose', size: 17 })),
          h('button', { type: 'button', className: 'merlink-expand', onClick: toggleSidebar, title: 'Expand sidebar', 'aria-label': 'Expand sidebar' },
            /* Intentionally explicit panel-right symbol for the collapsed logo's hover affordance. */
            h('svg', { width: '24', height: '24', viewBox: '0 0 24 24', fill: 'none', stroke: 'currentColor', strokeWidth: '2', strokeLinecap: 'round', strokeLinejoin: 'round', 'aria-hidden': 'true' }, h('rect', { x: '3', y: '3', width: '18', height: '18', rx: '2' }), h('path', { d: 'M15 3v18' }))
          )
        ),
        h('span', { className: 'merlink-nav-label' }, 'Workspace'),
        h('nav', { className: 'merlink-nav', 'aria-label': 'Workspace' }, ...navigation.map(route => h('a', { key: route.key, href: route.href, className: 'merlink-nav-link', 'aria-current': active === route.key || active === route.href ? 'page' : undefined, 'data-tooltip': route.label, 'aria-label': route.label, title: route.label }, h(Icon, { name: route.icon, size: 18 }), h('span', null, route.label)))),
        sidebarExtra && h('div', { className: 'merlink-sidebar-extra' }, sidebarExtra),
        sidebarExtra && collapsed && h('button', { type:'button', className:'merlink-history-expand', title:'Expand sidebar to view conversations', 'aria-label':'Expand sidebar to view conversations', onClick:() => setCollapsed(false) }, h(Icon, { name:'MessagesSquare', size:18 })),
        sidebarDrawer && h('button', { type:'button', className:'merlink-drawer-close', 'aria-label':'Close conversations', onClick:onCloseSidebarDrawer }, h(Icon, { name:'X', size:18 })),
        h('div', { className: 'merlink-sidebar-bottom' }, h('button', { type: 'button', className: 'merlink-nav-link merlink-settings-link', title: 'Settings', 'aria-label': 'Settings', onClick: () => { setSettingsTab('general'); setSettingsOpen(true); onCloseSidebarDrawer?.(); } }, h(Icon, { name: 'Settings2', size: 18 }), h('span', null, 'Settings')), h('div', { className: 'merlink-local-mode', title: 'Local Mode — sample data on this device' }, h('i', { className: 'merlink-local-dot' }), h('div', { className: 'merlink-local-copy' }, h('strong', null, 'Local Mode'), h('span', null, 'All data stored on device'))))
      ),
      h('main', { className: `merlink-content${wide || noMaxWidth || fullWidth || active === 'chat' || active === 'chat.html' ? ' is-wide' : ''}` }, h('div', { className: 'merlink-content-inner' }, children)),
      h(SettingsDialog, { open: settingsOpen, initialTab: settingsTab, onClose: () => setSettingsOpen(false) })
    );
  }
  function Dialog({ open, onClose, title, children, wide = false }) {
    const panelRef = React.useRef(null);
    const closeRef = React.useRef(null);
    const onCloseRef = React.useRef(onClose);
    onCloseRef.current = onClose;
    React.useEffect(() => {
      if (!open) return;
      const previousOverflow = document.body.style.overflow;
      const previousFocus = document.activeElement;
      document.body.style.overflow = 'hidden';
      (closeRef.current || panelRef.current)?.focus();
      function onKeyDown(event) {
        const panel = panelRef.current;
        if (!panel) return;
        const dialogs = document.querySelectorAll('.merlink-dialog-backdrop');
        if (dialogs[dialogs.length - 1] !== panel.parentElement) return;
        if (event.key === 'Escape') {
          event.preventDefault();
          onCloseRef.current?.();
        } else if (event.key === 'Tab') {
          const selectors = 'a[href], button:not([disabled]), input:not([disabled]), select:not([disabled]), textarea:not([disabled]), [tabindex]:not([tabindex="-1"])';
          const focusable = [...panel.querySelectorAll(selectors)].filter(element => element.getClientRects().length && getComputedStyle(element).visibility !== 'hidden');
          if (!focusable.length) { event.preventDefault(); panel.focus(); return; }
          const first = focusable[0], last = focusable[focusable.length - 1];
          if (!panel.contains(document.activeElement)) { event.preventDefault(); (event.shiftKey ? last : first).focus(); }
          else if (event.shiftKey && document.activeElement === first) { event.preventDefault(); last.focus(); }
          else if (!event.shiftKey && document.activeElement === last) { event.preventDefault(); first.focus(); }
        }
      }
      document.addEventListener('keydown', onKeyDown);
      return () => {
        document.body.style.overflow = previousOverflow;
        document.removeEventListener('keydown', onKeyDown);
        if (previousFocus && previousFocus.isConnected && typeof previousFocus.focus === 'function') previousFocus.focus();
      };
    }, [open]);
    if (!open) return null;
    return global.ReactDOM.createPortal(h('div', { className: 'merlink-dialog-backdrop', onMouseDown: event => { if (event.target === event.currentTarget) onCloseRef.current?.(); } }, h('section', { ref: panelRef, tabIndex: -1, className: `dialog-panel${wide ? ' merlink-dialog-panel-wide' : ''}`, role: 'dialog', 'aria-modal': 'true', 'aria-label': title || 'Dialog' }, h('div', { className: 'merlink-dialog-head' }, h('h2', null, title), h('button', { ref: closeRef, type: 'button', className: 'btn merlink-dialog-close', onClick: () => onCloseRef.current?.(), 'aria-label': 'Close dialog' }, h(Icon, { name: 'X', size: 17 }))), h('div', { className: 'merlink-dialog-body' }, children))), document.body);
  }

  const PROVIDER_CATALOG = [
    {id:'openai',name:'OpenAI',baseURL:'https://api.openai.com/v1',models:['gpt-4o','gpt-4o-mini','o1','o1-mini']},
    {id:'anthropic',name:'Anthropic',baseURL:'https://api.anthropic.com',models:['claude-sonnet-4-5','claude-opus-4','claude-haiku-4']},
    {id:'deepseek',name:'DeepSeek',baseURL:'https://api.deepseek.com',models:['deepseek-chat','deepseek-coder','deepseek-reasoner']},
    {id:'google',name:'Google Gemini',baseURL:'https://generativelanguage.googleapis.com/v1',models:['gemini-2.5-pro','gemini-2.5-flash']},
    {id:'qwen',name:'Qwen (Alibaba)',baseURL:'https://dashscope.aliyuncs.com/compatible-mode/v1',models:['qwen-max','qwen-plus']},
    {id:'glm',name:'Zhipu GLM',baseURL:'https://open.bigmodel.cn/api/paas/v4',models:['glm-4-plus','glm-4']},
    {id:'zai',name:'Z.ai',baseURL:'https://api.z.ai/api/paas/v4',models:['glm-4.6']},
    {id:'kimi',name:'Kimi (Moonshot)',baseURL:'https://api.moonshot.cn/v1',models:['moonshot-v1-8k','moonshot-v1-32k']},
    {id:'command-code',name:'Command Code',baseURL:'',models:[]}
  ];
  /** @typedef {{id:string,name:string,contextWindow:string,maxTokens:string,inputTypes:{text:boolean,image:boolean},reasoningLevel:boolean}} ModelConfig */
  const MODEL_OVERRIDES = {
    'gpt-4o': {contextWindow:'128K',maxTokens:'4K',inputTypes:{text:true,image:true}},
    'claude-sonnet-4-5': {contextWindow:'200K',maxTokens:'8K',inputTypes:{text:true,image:true},reasoningLevel:true},
    'deepseek-reasoner': {reasoningLevel:true}
  };
  function modelConfig(model) {
    const value = typeof model === 'string' ? {id:model,name:model} : model && typeof model === 'object' ? model : {};
    const defaults = MODEL_OVERRIDES[value.id] || {};
    return {
      id:typeof value.id === 'string' ? value.id : '', name:typeof value.name === 'string' ? value.name : String(value.id || ''),
      contextWindow:value.contextWindow || defaults.contextWindow || '128K', maxTokens:value.maxTokens || defaults.maxTokens || '4K',
      inputTypes:{text:value.inputTypes?.text ?? defaults.inputTypes?.text ?? true,image:value.inputTypes?.image ?? defaults.inputTypes?.image ?? false},
      reasoningLevel:value.reasoningLevel ?? defaults.reasoningLevel ?? false
    };
  }
  function catalogModels(provider) { return (provider?.models || []).map(modelConfig); }
  const providerDefaults = () => ({ providers: PROVIDER_CATALOG.map(item => ({...item, models:catalogModels(item), kind:'builtin', status:'available', apiKey:'', apiProtocol:item.id === 'anthropic' ? 'anthropic' : 'openai', defaultModel:item.models[0] || ''})), defaultProviderId:null });
  function loadProviders() {
    try {
      const saved = JSON.parse(storage.get('merlink-llm-providers', 'null'));
      if (saved && Array.isArray(saved.providers)) return {...saved,providers:saved.providers.map(provider => ({...provider,models:Array.isArray(provider.models) ? provider.models.map(modelConfig) : [],apiProtocol:provider.apiProtocol || (provider.id === 'anthropic' ? 'anthropic' : 'openai')}))};
    } catch (_) {}
    return providerDefaults();
  }
  function SettingsDialog({open, onClose, initialTab = 'general'}) {
    const [tab, setTab] = React.useState(initialTab);
    const [language, setLanguage] = React.useState(() => storage.get('merlink-language', 'en'));
    const {theme, setTheme} = useTheme();
    const [data, setData] = React.useState(loadProviders);
    const [editor, setEditor] = React.useState(null);
    const [form, setForm] = React.useState({});
    const [notice, setNotice] = React.useState('');
    const [expandedModel, setExpandedModel] = React.useState(-1);
    const [fetching, setFetching] = React.useState(false);
    const fetchTimer = React.useRef(null);
    React.useEffect(() => () => global.clearTimeout(fetchTimer.current), []);
    const closeEditor = () => { global.clearTimeout(fetchTimer.current); setFetching(false); setEditor(null); };
    const closeRef = React.useRef(null);
    const editorRef = React.useRef(null);
    const panelRef = React.useRef(null);
    const onCloseRef = React.useRef(onClose);
    onCloseRef.current = onClose;
    React.useEffect(() => { if (open) setTab(initialTab); }, [open, initialTab]);
    React.useEffect(() => {
      if (!open) return;
      const oldFocus = document.activeElement;
      const oldOverflow = document.body.style.overflow;
      document.body.style.overflow = 'hidden';
      closeRef.current?.focus();
      function keydown(event) {
        const panel = editorRef.current || panelRef.current;
        if (!panel) return;
        if (event.key === 'Escape') { event.preventDefault(); if (editorRef.current) closeEditor(); else onCloseRef.current?.(); return; }
        if (event.key !== 'Tab') return;
        const focusables = [...panel.querySelectorAll('button:not([disabled]), input:not([disabled]), select:not([disabled]), textarea:not([disabled]), a[href]')].filter(el => el.getClientRects().length && getComputedStyle(el).visibility !== 'hidden');
        if (!focusables.length) { event.preventDefault(); panel.focus(); return; }
        const first = focusables[0], last = focusables[focusables.length-1];
        if (!panel.contains(document.activeElement)) { event.preventDefault(); (event.shiftKey ? last : first).focus(); }
        else if (event.shiftKey && document.activeElement === first) { event.preventDefault(); last.focus(); }
        else if (!event.shiftKey && document.activeElement === last) { event.preventDefault(); first.focus(); }
      }
      document.addEventListener('keydown', keydown);
      return () => { document.body.style.overflow = oldOverflow; document.removeEventListener('keydown', keydown); if (oldFocus?.isConnected) oldFocus.focus(); };
    }, [open]);
    React.useEffect(() => { if (editor) editorRef.current?.querySelector('button[aria-label="Close provider editor"]')?.focus(); else if (open) closeRef.current?.focus(); }, [editor, open]);
    function newForm(source, mode) {
      const models = (source.models || []).map(modelConfig);
      return {id:source.id || '', name:source.name || '', baseURL:source.baseURL || '', apiKey:source.apiKey || '', apiProtocol:source.apiProtocol || (source.id === 'anthropic' ? 'anthropic' : 'openai'), models, defaultModel:source.defaultModel || models[0]?.id || '', showKey:false};
    }
    function openEditor(provider = null) {
      const source = provider || data.providers.find(item => item.id === 'openai') || PROVIDER_CATALOG[0];
      setForm(newForm(source, provider?.kind === 'custom' ? 'custom' : 'third-party'));
      setExpandedModel(-1);
      setEditor({id:provider?.id || null, mode:provider?.kind === 'custom' ? 'custom' : 'third-party'});
    }
    function switchProvider(id) {
      global.clearTimeout(fetchTimer.current);
      setFetching(false);
      const provider = data.providers.find(item => item.id === id) || PROVIDER_CATALOG.find(item => item.id === id);
      setForm(newForm(provider, 'third-party'));
      setExpandedModel(-1);
    }
    function update(field, value) { setForm(previous => ({...previous, [field]:value})); }
    function updateModel(index, patch) { setForm(previous => ({...previous, models:previous.models.map((model, position) => position === index ? {...model,...patch} : model)})); }
    function restoreModels() {
      global.clearTimeout(fetchTimer.current);
      setFetching(false);
      const catalog = PROVIDER_CATALOG.find(item => item.id === form.id);
      if (!catalog) return;
      const models = catalogModels(catalog);
      setForm(previous => ({...previous,models,defaultModel:models[0]?.id || ''}));
      setExpandedModel(-1);
    }
    function fetchModels() {
      if (fetching) return;
      setFetching(true);
      fetchTimer.current = global.setTimeout(() => {
        const prefix = form.id || 'custom';
        const available = [modelConfig({id:`${prefix}-preview`,name:`${form.name || prefix} Preview`}),modelConfig({id:`${prefix}-reasoning`,name:`${form.name || prefix} Reasoning`,reasoningLevel:true})];
        setForm(previous => ({...previous, models:[...previous.models,...available.filter(item => !previous.models.some(model => model.id === item.id))]}));
        setFetching(false);
      },1500);
    }
    const models = Array.isArray(form.models) ? form.models : [];
    const custom = editor?.mode === 'custom';
    const validModels = models.length > 0 && models.every(model => model.id.trim() && model.name.trim()) && new Set(models.map(model => model.id.trim())).size === models.length;
    const valid = Boolean(form.apiKey?.trim() && validModels && (!custom || form.name?.trim() && /^https?:\/\/\S+/i.test(form.baseURL || '')));
    function save() {
      if (!valid) return;
      const id = custom ? (editor.id || `custom-${Date.now()}`) : form.id;
      const catalog = PROVIDER_CATALOG.find(item => item.id === id);
      const existing = data.providers.find(item => item.id === id);
      const cleaned = models.map(model => modelConfig({...model,id:model.id.trim(),name:model.name.trim()}));
      const defaultModel = cleaned.some(model => model.id === form.defaultModel) ? form.defaultModel : cleaned[0].id;
      const provider = {id, name:form.name.trim() || catalog?.name || id, kind:custom ? 'custom' : 'builtin', status:'configured', apiKey:form.apiKey.trim(), baseURL:form.baseURL.trim() || catalog?.baseURL || '', apiProtocol:form.apiProtocol || 'openai', models:cleaned, defaultModel};
      const providers = existing ? data.providers.map(item => item.id === id ? provider : item) : [...data.providers, provider];
      const next = {providers, defaultProviderId:data.defaultProviderId || id};
      storage.set('merlink-llm-providers', JSON.stringify(next)); setData(next); closeEditor(); setNotice('Provider saved');
    }
    function removeProvider(id) {
      const next = {...data, providers:data.providers.filter(item => item.id !== id), defaultProviderId:data.defaultProviderId === id ? null : data.defaultProviderId};
      storage.set('merlink-llm-providers', JSON.stringify(next)); setData(next); setNotice('Provider deleted');
    }
    function clearKey() {
      const next = {...data, providers:data.providers.map(item => item.id === form.id ? {...item, apiKey:'', status:'available'} : item)};
      storage.set('merlink-llm-providers', JSON.stringify(next)); setData(next); closeEditor(); setNotice('Saved API key cleared');
    }
    const field = (label, element) => h('label', {className:'merlink-settings-field'}, h('span',null,label), element);
    const modelField = (label, value, placeholder, change) => field(label,h('input',{value,placeholder,onChange:event=>change(event.target.value)}));
    function renderModels() {
      return h('section',{className:'merlink-model-section','aria-label':'Customized model catalog'},
        h('div',{className:'merlink-model-heading'},h('div',null,h('h3',null,'Models'),h('span',{className:'merlink-settings-hint'},'Customized model catalog')),
          h('div',{className:'merlink-model-tools'},h('button',{type:'button',disabled:custom || fetching,onClick:restoreModels},'Restore defaults'),h('button',{type:'button',disabled:fetching,onClick:fetchModels},fetching ? 'Fetching models…' : 'Fetch available models'))),
        h('div',{className:'merlink-model-list'},...models.map((model,index)=>h('div',{key:index,className:'merlink-model-row'},
          h('div',{className:'merlink-model-row-top'},h('div',{className:'merlink-model-inputs'},
            modelField('Model ID',model.id,'deepseek-v4',value=>updateModel(index,{id:value})),
            modelField('Display Name',model.name,'DeepSeek V4',value=>updateModel(index,{name:value}))),
            h('div',{className:'merlink-model-row-actions'},h('button',{type:'button','aria-label':`${expandedModel === index ? 'Collapse' : 'Expand'} model ${model.id || index + 1}`,'aria-expanded':expandedModel === index,onClick:()=>setExpandedModel(expandedModel === index ? -1 : index)},h(Icon,{name:expandedModel === index ? 'ChevronUp' : 'ChevronDown',size:17})),
              h('button',{type:'button','aria-label':`Delete model ${model.id || index + 1}`,onClick:()=>{setForm(previous=>({...previous,models:previous.models.filter((_,position)=>position !== index)}));setExpandedModel(-1);}},h(Icon,{name:'Trash2',size:17})))),
          expandedModel === index && h('div',{className:'merlink-model-expanded'},h('div',{className:'merlink-provider-custom-fields'},
            modelField('Context window',model.contextWindow,'128K',value=>updateModel(index,{contextWindow:value})),
            modelField('Max output tokens',model.maxTokens,'4K',value=>updateModel(index,{maxTokens:value}))),
            h('div',{className:'merlink-model-checks'},h('strong',null,'Input types'),...[['text','Text'],['image','Image']].map(([key,label])=>h('label',{key},h('input',{type:'checkbox',checked:Boolean(model.inputTypes[key]),onChange:event=>updateModel(index,{inputTypes:{...model.inputTypes,[key]:event.target.checked}})}),label))),
            h('label',{className:'merlink-model-switch'},h('input',{type:'checkbox',role:'switch',checked:Boolean(model.reasoningLevel),onChange:event=>updateModel(index,{reasoningLevel:event.target.checked})}),'Reasoning levels'))))),
        h('button',{type:'button',className:'merlink-model-add',onClick:()=>{setForm(previous=>({...previous,models:[...previous.models,modelConfig({id:'',name:''})]}));setExpandedModel(models.length);}},h(Icon,{name:'Plus',size:16}),'Add model'),
        fetching && h('p',{className:'merlink-model-fetch-status',role:'status'},'Loading mock model catalog…'),
        !validModels && h('p',{className:'merlink-model-fetch-status'},'Add at least one model with a unique ID and display name.'));
    }
    if (!open) return null;
    return global.ReactDOM.createPortal(h('div', {className:'merlink-settings-backdrop', onMouseDown:event => {if (event.target === event.currentTarget && !editor) onCloseRef.current?.();}},
      h('section', {ref:panelRef, tabIndex:-1, role:'dialog', 'aria-modal':'true', 'aria-label':'Settings', className:'merlink-settings-panel'},
        h('nav', {className:'merlink-settings-tabs', 'aria-label':'Settings sections'},
          h('div', {className:'merlink-settings-brand'}, h(Icon,{name:'Settings2',size:20}), 'Settings'),
          ...[['general','General','Settings'],['llm-provider','LLM Provider','Bot']].map(([id,label,icon]) => h('button',{key:id,type:'button',className:`merlink-settings-tab${tab === id ? ' is-active' : ''}`, onClick:()=>setTab(id)},h(Icon,{name:icon,size:18}),label))
        ),
        h('div', {className:'merlink-settings-content'},
          h('header',{className:'merlink-settings-head'},h('span',null,tab === 'general' ? 'GENERAL' : 'LLM PROVIDER'),h('button',{ref:closeRef,type:'button',className:'merlink-settings-icon',onClick:()=>onCloseRef.current?.(),'aria-label':'Close settings'},h(Icon,{name:'X',size:19}))),
          h('div',{className:'merlink-settings-body'},tab === 'general' ?
            h(React.Fragment,null,h('h2',null,'Appearance'),h('p',null,'Make your writing space feel like yours.'),
              field('Theme',h('div',{className:'merlink-theme-options'}, ...[['light','Light','Sun'],['dark','Dark','Moon'],['system','System','Monitor']].map(([value,label,icon])=>h('button',{key:value,type:'button',className:theme === value ? 'is-active' : '',onClick:()=>setTheme(value),'aria-pressed':theme === value},h(Icon,{name:icon,size:17}),label)))),
              h('h2',null,'Language'),field('Language',h('select',{value:language,onChange:event=>{setLanguage(event.target.value);storage.set('merlink-language',event.target.value);}},...[['en','English'],['vi','Tiếng Việt'],['ja','日本語'],['zh-Hans','简体中文'],['zh-Hant','繁體中文']].map(([value,label])=>h('option',{key:value,value},label)))),
              h('p',{className:'merlink-settings-hint'},'Language preference is saved; translation is coming later.'),
              h('div',{className:'merlink-settings-about'},h('strong',null,'Merlink Harness · 0.1.0-alpha'),h('span',null,'A local-first writing studio.')))
            : h(React.Fragment,null,h('h2',null,'Model providers'),h('p',null,'Manage connections on this device. API keys are stored in browser localStorage; use test credentials in this demo.'),
              h('div',{className:'merlink-provider-list'},...data.providers.map(provider=>h('div',{key:provider.id,className:'merlink-provider-row',role:'button',tabIndex:0,onClick:()=>openEditor(provider),onKeyDown:event=>{if(event.key === 'Enter' || event.key === ' ') {event.preventDefault();openEditor(provider);}}},
                h('span',{className:'merlink-provider-avatar'},provider.name.slice(0,1)),h('span',{className:'merlink-provider-details'},h('strong',null,provider.name,provider.kind === 'custom' && h('small',null,'Custom')),h('span',null,`${provider.models.length} models configured · ${provider.models.find(model=>model.id === provider.defaultModel)?.name || provider.models[0]?.name || 'No model'}`)),
                h('span',{className:`merlink-provider-status ${provider.apiKey ? 'is-configured' : ''}`,title:provider.apiKey ? 'Configured' : 'Available'}),
                h('button',{type:'button',className:'merlink-settings-icon',onClick:event=>{event.stopPropagation();openEditor(provider);},'aria-label':`Edit ${provider.name}`},h(Icon,{name:'Pencil',size:17})),
                provider.kind === 'custom' && h('button',{type:'button',className:'merlink-settings-icon',onClick:event=>{event.stopPropagation();removeProvider(provider.id);},'aria-label':`Delete ${provider.name}`},h(Icon,{name:'Trash2',size:17}))))),
              h('button',{type:'button',className:'merlink-provider-add',onClick:()=>openEditor()},h(Icon,{name:'Plus',size:17}),'Add model provider'),notice && h('p',{role:'status',className:'merlink-settings-hint'},notice)))
        )
      ),
      editor && h('div',{className:'merlink-provider-backdrop',onMouseDown:event=>{if(event.target === event.currentTarget)closeEditor();}}, 
        h('section',{ref:editorRef,tabIndex:-1,role:'dialog','aria-modal':'true','aria-label':'Edit model provider',className:'merlink-provider-editor'},
          h('header',{className:'merlink-settings-head'},h('strong',null,editor.id ? 'Edit model provider' : 'Add model provider'),h('button',{type:'button',className:'merlink-settings-icon','aria-label':'Close provider editor',onClick:closeEditor},h(Icon,{name:'X',size:18}))),
          h('div',{className:'merlink-provider-editor-body'},
            h('div',{className:'merlink-provider-modes'},...[['third-party','Third-party model provider'],['custom','Custom model API']].map(([mode,label])=>h('button',{key:mode,type:'button',className:editor.mode === mode ? 'is-active' : '',onClick:()=>{global.clearTimeout(fetchTimer.current);setFetching(false);setExpandedModel(-1);setEditor({id:null,mode});setForm(mode === 'custom' ? newForm({id:'',name:'',baseURL:'',models:[]},mode) : newForm(data.providers.find(item=>item.id === 'openai') || PROVIDER_CATALOG[0],mode));}},label))),
            h('p',null,custom ? 'Connect to any OpenAI-compatible endpoint.' : 'Pick OpenAI, Anthropic, Kimi, or another provider and enter its API key.'),
            !custom && field('Provider',h('select',{value:form.id || 'openai',onChange:event=>switchProvider(event.target.value)},...PROVIDER_CATALOG.map(item=>h('option',{key:item.id,value:item.id},item.name)))),
            field('API key',h('span',{className:'merlink-key-input'},h('input',{type:form.showKey ? 'text' : 'password',value:form.apiKey || '',required:true,placeholder:'Enter an API key',onChange:event=>update('apiKey',event.target.value)}),h('button',{type:'button','aria-label':form.showKey ? 'Hide API key' : 'Reveal API key',onClick:()=>update('showKey',!form.showKey)},h(Icon,{name:form.showKey ? 'EyeOff' : 'Eye',size:17})))),
            h('details',{className:'merlink-provider-advanced',open:custom ? true : undefined},h('summary',null,'Customized settings'),
              h('div',{className:'merlink-provider-custom-fields'},
                field('Display name',h('input',{value:form.name || '',required:custom,placeholder:'Provider name',onChange:event=>update('name',event.target.value)})),
                field('Base URL',h('input',{type:'url',value:form.baseURL || '',required:custom,placeholder:PROVIDER_CATALOG.find(item=>item.id === form.id)?.baseURL || 'https://api.example.com/v1',onChange:event=>update('baseURL',event.target.value)}))),
              field('API protocol',h('select',{value:form.apiProtocol || 'openai',onChange:event=>update('apiProtocol',event.target.value)},h('option',{value:'openai'},'OpenAI Chat Completions'),h('option',{value:'anthropic'},'Anthropic Messages API'),h('option',{value:'custom'},'Custom'))),
              field('Default model',h('select',{value:models.some(model=>model.id === form.defaultModel) ? form.defaultModel : models[0]?.id || '',onChange:event=>update('defaultModel',event.target.value)},...models.filter(model=>model.id).map((model,index)=>h('option',{key:`${index}-${model.id}`,value:model.id},model.name || model.id)))),
              renderModels()),
            h('footer',{className:'merlink-provider-editor-actions'},!custom && data.providers.some(item=>item.id === form.id && item.apiKey) && h('button',{type:'button',className:'btn',onClick:clearKey},'Clear API key'),h('button',{type:'button',className:'btn',onClick:closeEditor},'Cancel'),h('button',{type:'button',className:'btn btn-primary',disabled:!valid,onClick:save},'Apply'))
          )
        )
      )
    ),document.body);
  }

  const projectSeeds = [
    { id:'glass-tide', title:'The Glass Tide', genre:'Literary fantasy', motifs:['Memory','Tides','Belonging'], cover:'linear-gradient(145deg,#29495b 0%,#7696a8 48%,#d9b8aa 100%)', status:'In progress', updatedAt:'2 hours ago', wordCount:42680, description:'On a coast where the sea returns forgotten memories, a mapmaker searches for the truth her family buried.', chapters:['The Shore of Lost Things','A Map Drawn in Salt','What the Water Kept','The Lighthouse Keeper'] },
    { id:'midnight-orchard', title:'The Midnight Orchard', genre:'Magical realism', motifs:['Family','Seasons','Secrets'], cover:'linear-gradient(145deg,#252d4e 0%,#826588 52%,#e8b58b 100%)', status:'In progress', updatedAt:'Yesterday', wordCount:28140, description:'Every winter, a forgotten orchard blooms at midnight; this year, its fruit bears the names of the missing.', chapters:['A Harvest Out of Season','Names Beneath the Bark','The House on Marigold Lane'] },
    { id:'atlas-of-small-worlds', title:'Atlas of Small Worlds', genre:'Science fiction', motifs:['Discovery','Home','Wonder'], cover:'linear-gradient(145deg,#253c5e 0%,#64899f 53%,#d1d6bd 100%)', status:'Draft', updatedAt:'3 days ago', wordCount:18420, description:'A cartographer of impossible spaces discovers a city folded between two pages of her father’s atlas.', chapters:['The Door in the Margin','Coordinates for Elsewhere','A City the Size of a Promise'] },
    { id:'after-the-lanterns', title:'After the Lanterns', genre:'Contemporary', motifs:['Second chances','Music','City life'], cover:'linear-gradient(145deg,#665269 0%,#bd8581 48%,#edc8a6 100%)', status:'Revising', updatedAt:'5 days ago', wordCount:56320, description:'Two estranged musicians meet beneath the last lantern festival in their hometown, a decade after their final song.', chapters:['The Last Train Home','An Unfinished Melody','The Space Between Notes'] },
    { id:'paper-constellations', title:'Paper Constellations', genre:'Young adult', motifs:['Friendship','Stars','Courage'], cover:'linear-gradient(145deg,#283561 0%,#747caf 52%,#c3a7a5 100%)', status:'Draft', updatedAt:'1 week ago', wordCount:12950, description:'A shy teenager makes constellations from the letters her friends never sent, and finds a map to a new beginning.', chapters:['Stars in an Envelope','The Observatory Club','Messages Without Addresses'] },
    { id:'the-quiet-archive', title:'The Quiet Archive', genre:'Mystery', motifs:['Truth','Archives','Inheritance'], cover:'linear-gradient(145deg,#39443f 0%,#85938a 50%,#d2bba2 100%)', status:'Complete', updatedAt:'2 weeks ago', wordCount:72410, description:'An archivist inherits a locked library containing every letter her town tried to forget.', chapters:['The Unmarked Key','Catalog of Silences','A Letter in Blue Ink'] }
  ];
  const beats = [
    ['The morning began with a small impossibility that no one else seemed to notice.', 'By noon, the familiar streets had acquired the uncertain edges of a place remembered incorrectly.', 'It was not the discovery itself that frightened her; it was how long the evidence had been waiting in plain sight.', 'Before evening, she understood that the next decision would change the story she had been telling herself.'],
    ['She returned to the place she had sworn she would never visit again, carrying only a notebook and an unfinished question.', 'A stranger at the door knew her name, but not the version of it she had used for years.', 'Their conversation moved carefully around the truth, like two people making room for a sleeping child.', 'When the light shifted, she saw the mark in the wall and realized the past had left more than a memory.'],
    ['For a moment the world held still, as if waiting to see which story they would choose to believe.', 'He laid the evidence between them; neither of them reached for it at first.', 'Outside, ordinary life continued with a confidence that felt almost cruel.', 'The answer did not make either of them brave, but it gave them somewhere to begin.'],
    ['There were three ways to cross the distance, and each demanded a different kind of honesty.', 'She chose the slow road because it left room to listen for the voices she had mistaken for silence.', 'At the edge of the city, the first sign appeared exactly where the old notes said it would not.', 'She folded the map, lifted her eyes, and followed the path that had never been drawn.']
  ];
  const projectDetails = [
    ['Mara','the returning tide','her mother’s map','the lighthouse bell'],
    ['Iris','the midnight orchard','her grandmother’s ledger','a pear with a name carved into its skin'],
    ['Leonie','the folded city','her father’s atlas','the paper door'],
    ['Sofia','the lantern-lit station','the recording they never released','a familiar melody'],
    ['Nell','the rooftop observatory','her best friend’s letter','the penciled stars'],
    ['Ada','the silent archive','the sealed correspondence','the blue-ink note']
  ];
  const MOCK_PROJECTS = projectSeeds.map((seed, projectIndex) => {
    const [person, place, object, signal] = projectDetails[projectIndex];
    return { ...seed, author: 'You', createdAt: ['2026-08-12','2026-08-29','2026-09-05','2026-06-18','2026-09-21','2026-05-10'][projectIndex], chapters: seed.chapters.map((title, chapterIndex) => ({
      id: `${seed.id}-chapter-${chapterIndex+1}`, title, wordCount: [3210,4110,2950,3820][chapterIndex], status: chapterIndex === seed.chapters.length-1 ? 'Draft' : 'Written',
      content: beats[chapterIndex].map((paragraph, paragraphIndex) => {
        const additions = [
          `For ${person}, ${place} had always been a landmark on the edge of childhood; now it seemed to be asking a question. ${paragraph} The air carried a trace of rain, and somewhere beyond the buildings ${signal} gave the hour a shape she could hold.`,
          `She unwrapped ${object} with the care of someone handling a fragile answer. ${paragraph} Every line suggested a different route through what she had been told, and every omission felt more deliberate than the last.`,
          `The thought brought ${person} back to ${place}, where ordinary details had begun to feel like clues. ${paragraph} She wrote the moment down before she could persuade herself it had meant nothing.`,
          `That night ${signal} returned to her thoughts. ${paragraph} She put ${object} beside the window, not because it solved anything, but because she wanted to remember where the search had truly begun.`
        ];
        return additions[paragraphIndex] + ` She noticed the small material details that gave the moment its weight: the color of the light on the table, the shape of a shadow beside the door, the pause before someone chose a word. None of them proved anything alone, but together they made it impossible to pretend this day was ordinary.`;
      })
    })) };
  });
  /* Complete opening chapters: individually written sample prose, not placeholder text. */
  const glassTideChapters = [
    [
      `The ferry entered Bellwether Harbor beneath a low ceiling of cloud. Mara stood at the rail with one hand on her suitcase and watched the shoreline assemble itself from familiar pieces: the net shed with its green roof, the crooked ladder on the quay, the lighthouse holding its pale head above the rain. Ten years away ought to have made the place smaller. Instead, she felt as though she had shrunk to the size of the girl who had once counted waves here for her mother. When the bell rang across the water, a boy on the deck looked up. There was no wind to carry its sound.`,
      `Her mother’s notebook lay at the top of the case because Mara had packed it last. The cover had softened at the corners, and the penciled coastlines inside no longer matched the shore in front of her. That had always been true of Bellwether: every storm changed the shape of a beach, and every spring someone moved a boundary stone. Still, one little mark bothered her. On the newest map, her mother had drawn a path to a cove that appeared nowhere else in the family papers. Beside it were three words pressed hard enough to dent the page: wait for the tide.`,
      `At the end of the gangway she stopped to retie a lace that did not need tying. The sand beneath the boards was strewn with sea glass, weed, and the ordinary wreckage of a high tide. Among it lay a blue button with a nick on one edge. Mara picked it up before she had quite decided to move. The button had come from a coat she had worn at thirteen, the winter her father went missing. She remembered losing it on the morning they told her he would not come back. For a long moment she could hear nothing except water moving through the stones.`
    ],
    [
      `Mara spread the maps over the kitchen table, weighing each corner with a cup. Her mother had dated every sheet in a tidy hand, yet the sequence made less sense the longer she studied it. A sandbar disappeared one October and returned the next May, a street bent west when every survey placed it east, and the little cove acquired a new name in blue ink. Saltmere. She ran a finger along its edge and felt a slight ridge beneath the paper. Her mother had folded the page there so often that it remembered the crease, even after years in a drawer.`,
      `Elias arrived with a tin of tea and the patient expression of someone who had been called without being invited. He set the tin down when he saw the blue button beside the maps. Neither of them spoke of it. Instead, he asked whether Mara had slept, whether the old stove still smoked in the mornings, whether she had found the spare key. His questions were kind, and their careful order made her wary. When she pointed to Saltmere, he glanced toward the window rather than at the paper. The sea was hidden by houses, but Mara had the feeling he could see it anyway.`,
      `They walked out at dusk when the tide began to draw away from the pier. Elias carried a lantern he did not light. The path on the map crossed a patch of empty shore, then turned between two black rocks whose shapes Mara knew by heart. At low water, another path appeared: a seam of pale shells leading to the cove. She had played here as a child and never seen it. Elias said the shoreline changed. Mara thought of her mother's heavy pencil strokes, the button waiting in the sand, and all the years people had told her that remembering was not the same as knowing.`
    ],
    [
      `Saltmere was not large enough to earn a name on most maps. Its stones were smooth and dark, its water colder than the harbor even in summer. Mara knelt beside a shallow pool where the retreating tide had left a child's red ribbon. The color was bright despite the salt, and a knot near its end matched the one her mother had taught her to tie. She had seen that ribbon in a photograph from the year before her father vanished. She looked at Elias, who was standing farther up the beach. He watched her hands but did not ask what she had found.`,
      `A sound came across the water then: not a voice, exactly, but the half-remembered rhythm of someone calling her in for supper. Mara shut her eyes. For an instant she was twelve again, barefoot on the kitchen floor, trying to hide sand inside her pockets. The memory arrived with its own weather, its own smell of onions browning in a pan. Then the sensation passed and she was kneeling among wet stones, holding a ribbon she could not explain. Elias crouched beside her and said that sometimes the sea returned what people had lost. He did not say whether he meant objects or something else.`,
      `She asked him how long he had known. Elias took time to answer, turning a pebble over in his palm until its dry side was dark. His grandfather had kept a ledger of everything washed up at Saltmere, he said. Most entries were ordinary: a key, a boot, a page torn from a cookbook. Beside a few, however, he had written names and dates that belonged to no shipwreck. Mara's father appeared in that ledger twice. The first entry was from before he went missing. The second, Elias said, had been written only last winter. The tide advanced a little and touched the toe of Mara's shoe.`
    ],
    [
      `The lighthouse smelled of oil, stone dust, and tea steeped too long. Elias led Mara past the narrow sleeping room to a cabinet built into the curved wall. Its lock opened with a key on a length of blue cord. Inside were rows of thin books, their spines labeled by year. She found the winter of her father's disappearance and ran her thumb over the uneven paper before turning a page. Most entries were written in Elias's grandfather's square script. At the bottom of one sheet, a different hand had added a note: the bell must be rung before the water reaches the steps.`,
      `The final ledger was scarcely half full. Mara found her father's name beneath a date from eight months ago and read it three times. The entry described a brass compass, its needle fixed toward the open sea, and a folded scrap of music in the case. No one had claimed either item. Elias brought the compass from a drawer and placed it between them without a word. Mara recognized a small gouge near its hinge, the mark she had made by dropping it from the kitchen table as a child. She had believed it was buried with him. The needle did not move when she lifted the case.`,
      `From the gallery she could see the harbor lights beginning to appear, each one answering a house on the shore. Elias told her the bell was a warning, not a welcome. When certain things came back with the tide, people followed them into the water, hoping to reach whatever memory had been returned. His grandfather had lost a brother that way. Mara closed the compass and looked at the narrow strip of sand below the tower. She wanted to ask whether her father had followed something, or whether the sea had offered him a way home. Below them, the first wave reached the steps.`
    ]
  ];
  MOCK_PROJECTS[0].chapters.forEach((chapter, index) => { chapter.content = glassTideChapters[index]; });
  const MOCK_CHARACTERS = [
    {id:'mara-vale',name:'Mara Vale',role:'Protagonist',age:32,gender:'Woman',story:'glass-tide',tags:['Observant','Guarded','Resilient'],image:'linear-gradient(160deg,#416172,#94aeb8 48%,#ddc9bc)',bio:'A mapmaker who charts changing coastlines and avoids the one shoreline she grew up on.',backstory:'Mara returned to her island hometown after inheriting her mother’s notebooks. The last pages hint at a tide that carries memories instead of shells.',relationships:[{name:'Elias Reed',relation:'Old friend; keeper of the lighthouse'},{name:'Nora Vale',relation:'Mother; an unfinished conversation'}],dialogueCount:48},
    {id:'elias-reed',name:'Elias Reed',role:'Lighthouse keeper',age:35,gender:'Man',story:'glass-tide',tags:['Patient','Loyal','Secretive'],image:'linear-gradient(160deg,#314b59,#668c93 56%,#d2a995)',bio:'The keeper of the lighthouse at the edge of a coast that refuses to stay in place.',backstory:'Elias inherited the watch from his grandfather and knows why the bell rings on windless nights.',relationships:[{name:'Mara Vale',relation:'Childhood friend and unresolved first love'}],dialogueCount:32},
    {id:'iris-bell',name:'Iris Bell',role:'Protagonist',age:27,gender:'Woman',story:'midnight-orchard',tags:['Curious','Tender','Determined'],image:'linear-gradient(160deg,#4b4059,#a9808f 58%,#edc7a6)',bio:'A botanist who comes home to settle an estate and finds an impossible winter orchard.',backstory:'Iris grew up hearing stories of trees that blossomed only in darkness. She never expected the stories to speak her name.',relationships:[{name:'June Bell',relation:'Grandmother; keeper of the ledger'}],dialogueCount:41},
    {id:'june-bell',name:'June Bell',role:'The keeper',age:73,gender:'Woman',story:'midnight-orchard',tags:['Wise','Wry','Protective'],image:'linear-gradient(160deg,#465152,#a08f84 55%,#e3c8aa)',bio:'Iris’s grandmother left behind an orchard full of questions and a ledger full of clues.',backstory:'For fifty winters June tended the trees alone, protecting a bargain she had made long ago.',relationships:[{name:'Iris Bell',relation:'Granddaughter and heir'}],dialogueCount:23},
    {id:'leonie-park',name:'Leonie Park',role:'Cartographer',age:29,gender:'Woman',story:'atlas-of-small-worlds',tags:['Inventive','Restless','Brave'],image:'linear-gradient(155deg,#2e536a,#76a1ae 52%,#d9c5ad)',bio:'An exacting cartographer who discovers that one of her father’s maps is a place.',backstory:'Leonie was raised among instruments, atlases, and the strange promise that every blank space has a reason.',relationships:[{name:'Tomas Park',relation:'Father and first teacher'}],dialogueCount:36},
    {id:'sofia-morales',name:'Sofia Morales',role:'Musician',age:34,gender:'Woman',story:'after-the-lanterns',tags:['Expressive','Stubborn','Hopeful'],image:'linear-gradient(150deg,#604c63,#aa7b7b 52%,#e3b896)',bio:'A songwriter whose return home brings an unfinished song back into her life.',backstory:'Sofia left the city with one suitcase and a melody she could never quite complete.',relationships:[{name:'Theo Lin',relation:'Former bandmate and collaborator'}],dialogueCount:55},
    {id:'theo-lin',name:'Theo Lin',role:'Composer',age:36,gender:'Man',story:'after-the-lanterns',tags:['Measured','Kind','Nostalgic'],image:'linear-gradient(150deg,#344e5f,#8495a3 52%,#c5a48d)',bio:'A composer who never stopped saving a place for Sofia in every new arrangement.',backstory:'Theo stayed when everyone else left, rebuilding the old rehearsal room one instrument at a time.',relationships:[{name:'Sofia Morales',relation:'Former bandmate; unresolved collaborator'}],dialogueCount:39},
    {id:'nell-hart',name:'Nell Hart',role:'Student',age:17,gender:'Girl',story:'paper-constellations',tags:['Imaginative','Quiet','Loyal'],image:'linear-gradient(150deg,#3c4778,#8b90c1 55%,#e0b8ad)',bio:'A teenager who turns unsent letters into a night sky of possibilities.',backstory:'After moving schools, Nell started mapping the people she missed as stars she could still find.',relationships:[{name:'Ava Rowan',relation:'Best friend across two cities'}],dialogueCount:27},
    {id:'ada-quill',name:'Ada Quill',role:'Archivist',age:41,gender:'Woman',story:'the-quiet-archive',tags:['Precise','Skeptical','Compassionate'],image:'linear-gradient(150deg,#39483f,#8b9888 55%,#d6baa2)',bio:'An archivist who believes every record deserves a witness, especially the ones people hide.',backstory:'Ada inherited the oldest library in town, and a collection of letters cataloged under a date that never existed.',relationships:[{name:'Evelyn Quill',relation:'Aunt; former curator of the archive'}],dialogueCount:46}
  ];
  const firstMessages = [
    {id:'msg-1',role:'user',content:'I want to open The Glass Tide with Mara returning to the shore after many years. Make it atmospheric, but keep the emotion grounded.',time:'10:24 AM'},
    {id:'msg-2',role:'assistant',content:'The ferry came in through a veil of morning rain, and Mara recognized the lighthouse before she recognized the town. Its bell sounded once, though the harbor was still. She kept her hands around the worn edge of her mother’s notebook and waited for the gangway to lower.',time:'10:24 AM'},
    {id:'msg-3',role:'user',content:'That’s lovely. Could you introduce the idea that the sea returns things people have forgotten?',time:'10:26 AM'},
    {id:'msg-4',role:'assistant',content:'At her feet, the water left a small blue button in the sand. Mara knew it at once: the missing button from the coat she had worn the winter her father taught her to read the tides. She had not thought about that coat in twenty years. She looked back at the sea as if it had spoken.',time:'10:26 AM'},
    {id:'msg-5',role:'user',content:'Now give Elias a quiet entrance. He should know more than he says.',time:'10:28 AM'},
    {id:'msg-6',role:'assistant',content:'“It does that,” Elias said behind her. He had grown into the lighthouse keeper’s coat as though he had always belonged to it. Mara closed her fingers around the button. He did not ask what she was holding, and somehow that told her he already knew.',time:'10:28 AM'}
  ];
  const MOCK_CHATS = [
    {id:'chat-1',title:'Opening scene · The Glass Tide',projectId:'glass-tide',updatedAt:'Just now',messages:firstMessages},
    {id:'chat-2',title:'The keeper’s confession',projectId:'glass-tide',updatedAt:'Yesterday',messages:[{id:'c2-1',role:'user',content:'Help me find a believable way for Elias to reveal what he has been hiding.',time:'Yesterday'},{id:'c2-2',role:'assistant',content:'Let him answer a small practical question first. When Mara asks why the bell rings without wind, he can admit he has been ringing it himself whenever the tide gives something back.',time:'Yesterday'}]},
    {id:'chat-3',title:'Orchard chapter ideas',projectId:'midnight-orchard',updatedAt:'2 days ago',messages:[{id:'c3-1',role:'user',content:'What might Iris find among the trees after midnight?',time:'2 days ago'},{id:'c3-2',role:'assistant',content:'One pear bears a name she knows; another bears one she has never heard, written in the careful handwriting of her grandmother.',time:'2 days ago'}]},
    {id:'chat-4',title:'A city between the pages',projectId:'atlas-of-small-worlds',updatedAt:'4 days ago',messages:[{id:'c4-1',role:'user',content:'Describe the moment Leonie steps inside the atlas.',time:'4 days ago'},{id:'c4-2',role:'assistant',content:'The page did not turn; the room did. Suddenly the paper had weather, and the inked streets held the warmth of late afternoon.',time:'4 days ago'}]},
    {id:'chat-5',title:'Character voice · Sofia',projectId:'after-the-lanterns',updatedAt:'Last week',messages:[{id:'c5-1',role:'user',content:'What does Sofia say when she sees Theo again?',time:'Last week'},{id:'c5-2',role:'assistant',content:'“You kept the piano,” she says, because asking why he stayed would require admitting she wanted him to.',time:'Last week'}]}
  ];
  const MOCK_PROVIDERS = [
    {id:'anthropic',name:'Anthropic Claude',models:['Claude Sonnet 4.5','Claude Opus 4.1','Claude Haiku 4.5'],defaultModel:'Claude Sonnet 4.5',status:'Ready to connect',color:'#d59779',baseURL:'https://api.anthropic.com'},
    {id:'openai',name:'OpenAI ChatGPT',models:['GPT-5','GPT-4.1','GPT-4.1 mini'],defaultModel:'GPT-5',status:'Ready to connect',color:'#56ab95',baseURL:'https://api.openai.com/v1'},
    {id:'deepseek',name:'DeepSeek',models:['DeepSeek V3','DeepSeek R1'],defaultModel:'DeepSeek V3',status:'Ready to connect',color:'#667ed7',baseURL:'https://api.deepseek.com'},
    {id:'qwen',name:'Qwen',models:['Qwen3 Max','Qwen3 Plus','Qwen3 32B'],defaultModel:'Qwen3 Max',status:'Ready to connect',color:'#8d71bf',baseURL:'https://dashscope.aliyuncs.com/compatible-mode/v1'},
    {id:'glm',name:'GLM (Zhipu)',models:['GLM-4.5','GLM-4.5 Air','GLM-4.1V'],defaultModel:'GLM-4.5',status:'Ready to connect',color:'#a17bb5',baseURL:'https://open.bigmodel.cn/api/paas/v4'},
    {id:'zai',name:'ZAI',models:['GLM-4.5','GLM-4.5 Air'],defaultModel:'GLM-4.5',status:'Ready to connect',color:'#7289dc',baseURL:'https://api.z.ai/api/paas/v4'},
    {id:'gemini',name:'Google Gemini',models:['Gemini 2.5 Pro','Gemini 2.5 Flash','Gemini 2.0 Flash'],defaultModel:'Gemini 2.5 Pro',status:'Ready to connect',color:'#6599dd',baseURL:'https://generativelanguage.googleapis.com/v1beta'}
  ];
  const MOCK_STYLE_PRESETS = [
    {id:'literary',name:'Literary',icon:'BookOpen',description:'Measured, evocative language with room for subtext.',tags:['Reflective','Lyrical'],preview:'The rain knew the way home before she did.'},
    {id:'pulp-fiction',name:'Pulp Fiction',icon:'Zap',description:'A page-turning style with sharp stakes and brisk action.',tags:['Punchy','Dramatic'],preview:'The door opened. There was no time to ask who held the key.'},
    {id:'web-novel',name:'Web Novel',icon:'PanelsTopLeft',description:'Immersive installments with momentum at every turn.',tags:['Serialized','Engaging'],preview:'One more chapter, she promised. The city had other ideas.'},
    {id:'light-novel',name:'Light Novel',icon:'Sparkles',description:'Expressive character moments and playful dialogue.',tags:['Playful','Character-led'],preview:'“You always say that,” she said, smiling as the sky changed color.'},
    {id:'dark-fantasy',name:'Dark Fantasy',icon:'Moon',description:'Mythic atmosphere with shadows and moral complexity.',tags:['Mythic','Moody'],preview:'Beneath the ruined bells, a different kingdom listened.'},
    {id:'isekai',name:'Isekai',icon:'Compass',description:'Discovery, wonder, and new rules in another world.',tags:['Adventure','Wonder'],preview:'The station doors opened onto a forest with two suns.'}
  ];
  const MOCK_WRITING_RULES = [
    {id:'avoid-adverbs',title:'Avoid adverbs',description:'Choose vivid verbs instead of leaning on modifiers.',enabled:true},
    {id:'show-dont-tell',title:'Show, don’t tell',description:'Reveal feelings through action, detail, and dialogue.',enabled:true},
    {id:'no-cliches',title:'No clichés',description:'Favor precise imagery over familiar stock phrases.',enabled:true},
    {id:'limit-pov-shifts',title:'Limit POV shifts',description:'Stay anchored in a chosen narrator per scene.',enabled:true},
    {id:'consistent-tense',title:'Consistent tense',description:'Maintain verb tense across each scene.',enabled:true}
  ];
  const MOCK_TONE_VALUES = { formality:55, darkness:38, humor:28, descriptiveness:70, pacing:52 };
  global.MOCK_PROJECTS = MOCK_PROJECTS;
  global.MOCK_CHARACTERS = MOCK_CHARACTERS;
  global.MOCK_CHATS = MOCK_CHATS;
  global.MOCK_PROVIDERS = MOCK_PROVIDERS;
  global.MOCK_STYLE_PRESETS = MOCK_STYLE_PRESETS;
  global.MOCK_WRITING_RULES = MOCK_WRITING_RULES;
  global.MOCK_TONE_VALUES = MOCK_TONE_VALUES;
  global.Merlink = { Shell, Icon, ThemeToggle, Dialog, SettingsDialog, useTheme, useSidebar, mock: { projects:MOCK_PROJECTS, characters:MOCK_CHARACTERS, chats:MOCK_CHATS, providers:MOCK_PROVIDERS, presets:MOCK_STYLE_PRESETS, rules:MOCK_WRITING_RULES, toneValues:MOCK_TONE_VALUES }, navigation };
})(window);
