<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>get 5 day weather forecast of Jakarta Selatan</description>
   <name>Get_5DayForecast</name>
   <tag></tag>
   <elementGuidId>51931b97-b711-42ae-a682-d6edb24804ef</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>true</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <katalonVersion>10.2.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${BASE_URL}${APP_URL}?lat=${VALID_LATITUDE}&amp;lon=${VALID_LONGITUDE}&amp;appid=${API_KEY}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <validationSteps>
      <id>97f40b28-cf08-4b05-918a-091b15feca32</id>
      <name>RIGHT_SCHEMA</name>
      <type>JSON_SCHEMA</type>
      <dataType>FILE</dataType>
      <target>RESPONSE</target>
      <data>JSONSchemas/forecast_schema.json</data>
      <activate>true</activate>
   </validationSteps>
   <variables>
      <defaultValue>GlobalVariable.API_KEY</defaultValue>
      <description></description>
      <id>d29a07f7-90e0-4de1-baa7-e4307457f5e1</id>
      <masked>false</masked>
      <name>API_KEY</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.BASE_URL</defaultValue>
      <description></description>
      <id>42827d00-a3f1-4499-8689-9b919e212ac7</id>
      <masked>false</masked>
      <name>BASE_URL</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.FORECAST_URL</defaultValue>
      <description></description>
      <id>63d076eb-ce20-48f0-bd31-037c5036e256</id>
      <masked>false</masked>
      <name>APP_URL</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.VALID_LATITUDE</defaultValue>
      <description></description>
      <id>e51c650f-5699-4c02-97c4-db10d05c2228</id>
      <masked>false</masked>
      <name>VALID_LATITUDE</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.VALID_LONGITUDE</defaultValue>
      <description></description>
      <id>1eeb31b6-b719-40de-9c1f-dc2c7fcf0fe7</id>
      <masked>false</masked>
      <name>VALID_LONGITUDE</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
