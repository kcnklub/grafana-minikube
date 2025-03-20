# Grafana Minikube

This project is a learning ground for the self hosted Grafana OSS ecosystem.

## Project breakdown

### `./kubernetes/`
This folder holds spec files and scripts for deploying grafana components to kubernetes directly without any other tools besides `kubectl`

### `./applications/`
This folder contains a set of projects that are set up to be monitored by the grafana deployments in the other folders
