import { createFileRoute } from '@tanstack/react-router'
import { SqlEditor } from '../components/query/Editor'

export const Route = createFileRoute('/query')({
  component: QueryPage
})



function QueryPage(){

  return(
    <SqlEditor sqlType={'sql'} value={''} onChange={function (_value: string): void {
      throw new Error('Function not implemented.')
    } }/>
  )
}
