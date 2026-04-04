export default {
  common: {
    cancel: 'Cancel',
    delete: 'Delete',
    create: 'Create',
    creating: 'Creating...',
    refresh: 'Refresh',
    open: 'Open',
    download: 'Download',
    upload: 'Upload',
    search: 'Search',
    error: 'Error',
    disconnect: 'Disconnect',
    import: 'Import',
    export: 'Export',
    unknown: 'unknown',
    copyUrl: 'Copy URL',
  },

  titlebar: {
    title: 'Client S3',
    logo: 'S3',
  },

  auth: {
    title: 'Client S3',
    subtitle: 'Connect to your S3 storage',
    tabs: {
      connect: 'Connection',
      profiles: 'Profiles',
    },
    connect: {
      title: 'New connection',
      description: 'Choose a provider or configure manually',
      savedProfile: 'Saved profile',
      choosePlaceholder: 'Choose a profile...',
      newConnection: 'New connection',
      provider: 'S3 Provider',
      documentation: '{name} documentation',
      name: 'Connection name',
      namePlaceholder: 'My S3 server',
      endpoint: 'Endpoint',
      endpointPlaceholder: 'https://s3.example.com',
      region: 'Region',
      accessKey: 'Access Key ID',
      secretKey: 'Secret Access Key',
      secretReconnect: 'Enter the secret to reconnect',
      pathStyle: 'Path Style',
      remember: 'Remember this profile',
      connecting: 'Connecting...',
      submit: 'Connect',
      defaultName: 'S3 Connection',
      connectionError: 'Connection error',
    },
    profiles: {
      title: 'Saved profiles',
      description: 'Manage your saved connections',
      empty: 'No saved profiles.',
      emptyHint: 'Check "Remember this profile" when connecting.',
      deleted: 'Profile deleted',
      deleteTitle: 'Delete profile?',
      deleteDescription: 'This action will permanently delete this profile and its credentials from the system keyring.',
      exported: '{count} profile(s) exported',
      imported: '{count} profile(s) imported',
      invalidFormat: 'Invalid format',
    },
  },

  browser: {
    buckets: 'Buckets',
    createBucket: 'Create bucket',
    deleteBucket: 'Delete bucket',
    selectBucket: 'Select a bucket',
    selectBucketFirst: 'Select a bucket first',
    emptyFolder: 'Empty folder',

    table: {
      name: 'Name',
      size: 'Size',
      modified: 'Modified',
    },

    context: {
      newFolder: 'New folder',
      uploadFiles: 'Upload files',
      deleteSelection: 'Delete selection ({count})',
    },

    toast: {
      downloaded: '{name} downloaded',
      deleted: '{name} deleted',
      urlCopied: 'URL copied',
      itemsDeleted: 'Items deleted',
      filesUploaded: '{count} file(s) uploaded',
      bucketCreated: 'Bucket "{name}" created',
      folderCreated: 'Folder "{name}" created',
    },
  },

  dialogs: {
    createBucket: {
      title: 'New bucket',
      description: 'Name must be lowercase, no spaces (a-z, 0-9, -, .)',
      label: 'Bucket name',
      placeholder: 'my-bucket',
    },
    createFolder: {
      title: 'New folder',
      description: 'Create a folder in {location}',
      label: 'Folder name',
      placeholder: 'new-folder',
    },
  },

  preview: {
    download: 'Download',
    copyUrl: 'Copy URL',
    tabs: {
      preview: 'Preview',
      info: 'Info',
    },
    tooLarge: 'File too large for preview',
    videoUnsupported: 'Your browser does not support video playback.',
    audioUnsupported: 'Your browser does not support audio playback.',
    pdfTitle: 'PDF preview',
    unknownType: 'Preview not available for this file type',
    downloaded: '{name} downloaded',
    urlCopied: 'URL copied',
    copyError: 'Copy error',
    meta: {
      contentType: 'Content-Type',
      size: 'Size',
      lastModified: 'Last modified',
      etag: 'ETag',
      storageClass: 'Storage Class',
    },
  },

  command: {
    placeholder: 'Search files, buckets, actions...',
    searching: 'Searching...',
    noResults: 'No results.',
    groups: {
      files: 'Files',
      buckets: 'Buckets',
      actions: 'Actions',
      connection: 'Connection',
    },
    actions: {
      createBucket: 'Create bucket',
      newFolder: 'New folder',
      uploadFiles: 'Upload files',
      refresh: 'Refresh',
      goUp: 'Go up',
    },
  },

  dropOverlay: {
    title: 'Drop your files here',
    subtitle: 'Files will be uploaded to the current folder',
  },

  layout: {
    newFolder: 'New folder',
    upload: 'Upload',
    refresh: 'Refresh',
  },

  settings: {
    title: 'Settings',
    tabs: {
      general: 'General',
      sessions: 'Sessions',
    },
    general: {
      title: 'General',
      theme: 'Theme',
      themeDescription: 'Choose the interface theme',
      themeLight: 'Light',
      themeDark: 'Dark',
      themeSystem: 'System',
      language: 'Language',
      languageDescription: 'Choose the interface language',
      langFr: 'Francais',
      langEn: 'English',
    },
    sessions: {
      title: 'Sessions',
      description: 'Manage your saved connection profiles',
      empty: 'No saved profiles.',
      deleted: 'Profile deleted',
      deleteTitle: 'Delete profile?',
      deleteDescription: 'This action will permanently delete this profile and its credentials from the system keyring.',
    },
  },
}
