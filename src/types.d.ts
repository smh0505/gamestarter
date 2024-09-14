declare interface TauriResponse<T> {
  kind: string,
  message: string,
  result?: T
}

declare interface Folder {
  directory: string,
  items: Item[],
  have_parent: boolean
}

declare interface Item {
  path: string,
  name: string,
  is_dir: boolean
}
