export interface S3Provider {
  id: string
  name: string
  icon: string
  description: string
  endpoint: string
  region: string
  pathStyle: boolean
  regions?: { value: string; label: string }[]
  endpointTemplate?: string // e.g. "https://s3.{region}.scw.cloud"
  helpUrl?: string
  fields?: {
    accessKeyLabel?: string
    secretKeyLabel?: string
    accessKeyPlaceholder?: string
    secretKeyPlaceholder?: string
  }
}

export const providers: S3Provider[] = [
  {
    id: 'custom',
    name: 'Custom S3',
    icon: 'HardDrive',
    description: 'Serveur S3 compatible (MinIO, Ceph, etc.)',
    endpoint: '',
    region: 'us-east-1',
    pathStyle: true,
  },
  {
    id: 'aws',
    name: 'Amazon S3',
    icon: 'Cloud',
    description: 'Amazon Web Services S3',
    endpoint: 'https://s3.{region}.amazonaws.com',
    region: 'us-east-1',
    pathStyle: false,
    endpointTemplate: 'https://s3.{region}.amazonaws.com',
    helpUrl: 'https://docs.aws.amazon.com/general/latest/gr/s3.html',
    regions: [
      { value: 'us-east-1', label: 'US East (N. Virginia)' },
      { value: 'us-east-2', label: 'US East (Ohio)' },
      { value: 'us-west-1', label: 'US West (N. California)' },
      { value: 'us-west-2', label: 'US West (Oregon)' },
      { value: 'eu-west-1', label: 'EU (Ireland)' },
      { value: 'eu-west-2', label: 'EU (London)' },
      { value: 'eu-west-3', label: 'EU (Paris)' },
      { value: 'eu-central-1', label: 'EU (Frankfurt)' },
      { value: 'ap-southeast-1', label: 'Asia Pacific (Singapore)' },
      { value: 'ap-northeast-1', label: 'Asia Pacific (Tokyo)' },
    ],
    fields: {
      accessKeyLabel: 'Access Key ID',
      secretKeyLabel: 'Secret Access Key',
      accessKeyPlaceholder: 'AKIAIOSFODNN7EXAMPLE',
    },
  },
  {
    id: 'cloudflare-r2',
    name: 'Cloudflare R2',
    icon: 'Cloud',
    description: 'Stockage objet Cloudflare sans frais de sortie',
    endpoint: 'https://{account_id}.r2.cloudflarestorage.com',
    region: 'auto',
    pathStyle: false,
    helpUrl: 'https://developers.cloudflare.com/r2/api/s3/tokens/',
    fields: {
      accessKeyLabel: 'Access Key ID (R2 API Token)',
      secretKeyLabel: 'Secret Access Key',
      accessKeyPlaceholder: 'R2 Access Key ID',
    },
  },
  {
    id: 'scaleway',
    name: 'Scaleway',
    icon: 'Cloud',
    description: 'Object Storage Scaleway',
    endpoint: 'https://s3.{region}.scw.cloud',
    region: 'fr-par',
    pathStyle: false,
    endpointTemplate: 'https://s3.{region}.scw.cloud',
    helpUrl: 'https://www.scaleway.com/en/docs/storage/object/',
    regions: [
      { value: 'fr-par', label: 'Paris' },
      { value: 'nl-ams', label: 'Amsterdam' },
      { value: 'pl-waw', label: 'Warsaw' },
    ],
    fields: {
      accessKeyLabel: 'Access Key',
      secretKeyLabel: 'Secret Key',
    },
  },
  {
    id: 'backblaze-b2',
    name: 'Backblaze B2',
    icon: 'Cloud',
    description: 'Stockage cloud Backblaze B2 S3-compatible',
    endpoint: 'https://s3.{region}.backblazeb2.com',
    region: 'us-west-004',
    pathStyle: false,
    endpointTemplate: 'https://s3.{region}.backblazeb2.com',
    helpUrl: 'https://www.backblaze.com/docs/cloud-storage',
    regions: [
      { value: 'us-west-004', label: 'US West' },
      { value: 'eu-central-003', label: 'EU Central' },
    ],
    fields: {
      accessKeyLabel: 'Application Key ID',
      secretKeyLabel: 'Application Key',
    },
  },
  {
    id: 'digitalocean-spaces',
    name: 'DigitalOcean Spaces',
    icon: 'Cloud',
    description: 'Object Storage DigitalOcean',
    endpoint: 'https://{region}.digitaloceanspaces.com',
    region: 'nyc3',
    pathStyle: false,
    endpointTemplate: 'https://{region}.digitaloceanspaces.com',
    helpUrl: 'https://docs.digitalocean.com/products/spaces/',
    regions: [
      { value: 'nyc3', label: 'New York 3' },
      { value: 'sfo3', label: 'San Francisco 3' },
      { value: 'ams3', label: 'Amsterdam 3' },
      { value: 'sgp1', label: 'Singapore 1' },
      { value: 'fra1', label: 'Frankfurt 1' },
      { value: 'syd1', label: 'Sydney 1' },
    ],
    fields: {
      accessKeyLabel: 'Spaces Access Key',
      secretKeyLabel: 'Spaces Secret Key',
    },
  },
  {
    id: 'wasabi',
    name: 'Wasabi',
    icon: 'Cloud',
    description: 'Hot Cloud Storage Wasabi',
    endpoint: 'https://s3.{region}.wasabisys.com',
    region: 'eu-central-1',
    pathStyle: false,
    endpointTemplate: 'https://s3.{region}.wasabisys.com',
    helpUrl: 'https://docs.wasabi.com/docs/what-are-the-service-urls-for-wasabis-different-storage-regions',
    regions: [
      { value: 'us-east-1', label: 'US East 1 (Virginia)' },
      { value: 'us-east-2', label: 'US East 2 (Virginia)' },
      { value: 'us-west-1', label: 'US West 1 (Oregon)' },
      { value: 'eu-central-1', label: 'EU Central 1 (Amsterdam)' },
      { value: 'eu-central-2', label: 'EU Central 2 (Frankfurt)' },
      { value: 'eu-west-1', label: 'EU West 1 (London)' },
      { value: 'eu-west-2', label: 'EU West 2 (Paris)' },
      { value: 'ap-northeast-1', label: 'AP Northeast 1 (Tokyo)' },
      { value: 'ap-southeast-1', label: 'AP Southeast 1 (Singapore)' },
    ],
    fields: {
      accessKeyLabel: 'Access Key',
      secretKeyLabel: 'Secret Key',
    },
  },
  {
    id: 'ovh',
    name: 'OVHcloud',
    icon: 'Cloud',
    description: 'Object Storage OVHcloud S3 API',
    endpoint: 'https://s3.{region}.io.cloud.ovh.net',
    region: 'gra',
    pathStyle: false,
    endpointTemplate: 'https://s3.{region}.io.cloud.ovh.net',
    helpUrl: 'https://help.ovhcloud.com/csm/en-public-cloud-storage-s3-getting-started',
    regions: [
      { value: 'gra', label: 'Gravelines (France)' },
      { value: 'sbg', label: 'Strasbourg (France)' },
      { value: 'bhs', label: 'Beauharnois (Canada)' },
      { value: 'de', label: 'Frankfurt (Germany)' },
      { value: 'uk', label: 'London (UK)' },
      { value: 'waw', label: 'Warsaw (Poland)' },
    ],
    fields: {
      accessKeyLabel: 'Access Key',
      secretKeyLabel: 'Secret Key',
    },
  },
  {
    id: 'minio',
    name: 'MinIO',
    icon: 'HardDrive',
    description: 'MinIO self-hosted',
    endpoint: 'http://localhost:9000',
    region: 'us-east-1',
    pathStyle: true,
    fields: {
      accessKeyLabel: 'Access Key (Root User)',
      secretKeyLabel: 'Secret Key (Root Password)',
      accessKeyPlaceholder: 'minioadmin',
    },
  },
]

export function resolveEndpoint(template: string, region: string, extra?: Record<string, string>): string {
  let result = template.replace('{region}', region)
  if (extra) {
    for (const [key, value] of Object.entries(extra)) {
      result = result.replace(`{${key}}`, value)
    }
  }
  return result
}
