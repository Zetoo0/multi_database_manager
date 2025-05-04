import { createFileRoute } from '@tanstack/react-router'
import { MigrationForm } from '../../../components/forms/MigrationForm'

export const Route = createFileRoute('/migration/_migration/migrationPage')({
  component: MigrationPage,
})


function MigrationPage() {
  return (
    <div>
        <MigrationForm/>
    </div>
  )
}