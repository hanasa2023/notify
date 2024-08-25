export interface DefaultCardProps {
  icon: string
  taskRemain: number
  title: string
}

export interface ListCardData {
  iconColor?: string
  title: string
  taskRemain: number
}

export interface ListItemData {
  isFinish: boolean
  title: string
  listName: string
  tag?: string
  isFlag: boolean
}
