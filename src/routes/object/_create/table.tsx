import { createFileRoute, useNavigate } from '@tanstack/react-router'
import { useEffect } from 'react'
import { useHotkey } from '../../../hooks/hotkey/useHotkey'
import { useLayout } from '../../../providers/LayoutProvider'
import { ObjectSubMenu } from '../../../components/object/ObjectSubMenu'
import { ObjectSubGroup } from '../../../components/object/ObjectSubGroup'
import { ColumnsForm } from '../../../components/forms/ColumnsForm'
import { useObject } from '../../../providers/ObjectProvider'

export const Route = createFileRoute('/object/_create/table')({
  component: ObjectCreatePage,
})

function ObjectCreatePage() {
  const { /*objectCreationInfo,*/ editObject } = useObject()
  const navigate = useNavigate()
  const { setShowStatusBar, setShowTitleBar } = useLayout()
  const { /*formatted*/ } = useHotkey('closeSettings', () => {
    navigate({ to: '/' })
  })
  useEffect(() => {
    console.log('ObjectEdit: ', editObject)
    setShowStatusBar(false)
    setShowTitleBar(false)
    return () => {
      setShowStatusBar(true)
      setShowTitleBar(true)
    }
  }, [])

  try {
    return (
      <ObjectSubMenu title="Connections">
        <ObjectSubGroup title="Connection List">
          {/*<CreateEntityForm type={objectCreationInfo?.type || 'table'} />*/}
          <ColumnsForm onChange={(data) => console.log(data)} data={[]} />
        </ObjectSubGroup>
      </ObjectSubMenu>
    )
  } catch (e) {
    console.error('Error in Object Creation page:', e)
    return <div>Error loading ConnectionPage</div>
  }
}
