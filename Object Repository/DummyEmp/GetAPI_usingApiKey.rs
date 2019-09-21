<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GetAPI_usingApiKey</name>
   <tag></tag>
   <elementGuidId>4d7e28f5-8b1f-4ca1-b5d1-94f1df5b2e76</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>http://api.openweathermap.org/data/2.5/weather?q=${City}&amp;appid=${APIKey}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>'Pune'</defaultValue>
      <description></description>
      <id>9c9419d8-2265-4d33-9d9f-c9e41192440b</id>
      <masked>false</masked>
      <name>City</name>
   </variables>
   <variables>
      <defaultValue>'b83cf670e7784744c7f80cdc46135f11'</defaultValue>
      <description></description>
      <id>31a690bc-08df-4144-8a42-e62f814dbfc5</id>
      <masked>false</masked>
      <name>APIKey</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
