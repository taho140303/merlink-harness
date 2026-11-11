import js from '@eslint/js'
import globals from 'globals'
import reactHooks from 'eslint-plugin-react-hooks'
import reactRefresh from 'eslint-plugin-react-refresh'
import tseslint from 'typescript-eslint'

export default tseslint.config(
  {
    ignores: [
      'dist',
      'build',
      '.react-router',
      'node_modules',
      // Cargo output and Tauri codegen are generated, not reviewable source.
      'src-tauri/target',
      'src-tauri/gen',
      // Generated from the Rust DTOs by ts-rs; never edited by hand.
      'src/shared/backend/generated.ts',
    ],
  },
  {
    extends: [js.configs.recommended, ...tseslint.configs.recommended],
    files: ['**/*.{ts,tsx}'],
    languageOptions: {
      ecmaVersion: 2020,
      globals: globals.browser,
    },
    plugins: {
      'react-hooks': reactHooks,
      'react-refresh': reactRefresh,
    },
    rules: {
      ...reactHooks.configs.recommended.rules,
      'react-refresh/only-export-components': 'off',
      '@typescript-eslint/no-explicit-any': 'off',
      '@typescript-eslint/no-unused-vars': ['warn', { argsIgnorePattern: '^_' }],
    },
  },
  {
    files: ['src/**/*.tsx'],
    ignores: [
      // Intentional design-system exclusions: mock routes, decoration, or color data.
      'src/app/routes/config.tsx',
      'src/app/routes/worldbuilding.tsx',
      'src/shared/components/background/AuroraBackground.tsx',
      'src/core/components/chat/OrbSphere.tsx',
      'src/core/components/character/CreateCharacterDialog.tsx',
      // The white switch knob is an intentional exception; leave baseline lint enabled.
      'src/core/components/sidebar/SidebarHistory.tsx',
    ],
    rules: {
      'no-restricted-syntax': [
        'error',
        {
          selector: 'Literal[value=/text-\\[\\d+px\\]/]',
          message: 'Use an approved typography token; arbitrary pixel font sizes are forbidden.',
        },
        {
          selector: 'Literal[value=/rounded-\\[/]',
          message: 'Use an approved radius token; arbitrary rounded values are forbidden.',
        },
        {
          selector: 'Literal[value=/\\bshadow-(2xs|xs|sm|md|lg|xl|2xl|inner)\\b/]',
          message: 'Use the tier-overlay shadow token instead of a standalone shadow utility.',
        },
        {
          selector: 'Literal[value=/\\bbg-white\\b/]',
          message: 'Use a surface token instead of opaque white.',
        },
        {
          selector: 'Literal[value=/focus:outline-(hidden|none)/]',
          message: 'Use the focus-ring utility; do not remove the focus outline.',
        },
        {
          selector: 'Literal[value=/#[0-9a-fA-F]{3,8}/]',
          message: 'Use design-system color tokens instead of hex colors in TSX.',
        },
        {
          selector: 'Literal[value=/dark:[a-z:]*-(fg|surface|line|accent)/]',
          message: 'Design tokens adapt to the active theme; remove this dark: color override.',
        },
      ],
    },
  },
)
