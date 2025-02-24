-- Add migration script here

-- ADD missing column name in projects
ALTER TABLE projects ADD COLUMN project_name VARCHAR(255) NOT NULL FIRST;